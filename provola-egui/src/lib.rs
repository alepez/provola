mod app;

pub use app::Config as GuiOpt;
use app::ProvolaGuiApp;
use crossbeam_channel::{bounded, select, Receiver, Sender};
use eframe::epi::RepaintSignal;
use provola_core::{Action, Error, Source, TestDataIn, TestDataOut, WatchOptions, Watcher};
use provola_testrunners::{make_test_runner, TestRunnerInfo};
use std::{path::PathBuf, sync::Arc, thread, time::Duration};

struct Setup {
    opt: GuiOpt,
    repaint_signal: Arc<dyn RepaintSignal>,
}

enum ActionMessage {
    Setup(Setup),
    RunAll,
}

enum FeedbackMessage {
    Result(provola_core::TestResult),
    WatchedChanged,
}

type ActionSender = Sender<ActionMessage>;
type ActionReceiver = Receiver<ActionMessage>;
type FeedbackSender = Sender<FeedbackMessage>;
type FeedbackReceiver = Receiver<FeedbackMessage>;

struct Server {
    opt: Option<GuiOpt>,
    repaint_signal: Option<Arc<dyn RepaintSignal>>,
    action_r: ActionReceiver,
    feedback_s: FeedbackSender,
}

pub fn run(opt: GuiOpt) -> Result<(), Error> {
    // Create a channel between working thread and main event loop:
    let (action_s, action_r) = bounded(1000);
    let (feedback_s, feedback_r) = bounded(1000);

    let app = ProvolaGuiApp::new(opt, action_s, feedback_r);
    let native_options = eframe::NativeOptions::default();

    let mut server = Server {
        opt: None,
        repaint_signal: None,
        action_r,
        feedback_s,
    };

    thread::spawn(move || {
        log::debug!("start_working_thread, spawned");
        server.run();
    });

    eframe::run_native(Box::new(app), native_options)
}

impl Server {
    fn handle_message(&mut self, msg: Result<ActionMessage, crossbeam_channel::RecvError>) {
        match msg {
            Ok(ActionMessage::Setup(setup)) => {
                let new_opt = setup.opt;
                let new_repaint_signal = setup.repaint_signal;

                log::info!("Setup!");
                if let Some(file_to_watch) = &new_opt.watch {
                    // FIXME Make this thread stoppable (when file_to_watch changes)
                    Server::start_watch_thread(
                        file_to_watch.clone(),
                        self.feedback_s.clone(),
                        new_repaint_signal.clone(),
                    );
                }

                self.opt = Some(new_opt);
                self.repaint_signal = Some(new_repaint_signal);
            }
            Ok(ActionMessage::RunAll) => {
                log::debug!("Receive Message::RunAll");
                // TODO Give a feedback if run_once return an error
                Server::run_once(&self.opt, self.feedback_s.clone()).ok();
                if let Some(repaint_signal) = &self.repaint_signal {
                    repaint_signal.request_repaint();
                }
            }
            _ => {}
        }
    }

    fn run(&mut self) {
        log::debug!("run_forever");

        loop {
            select! {
                recv(self.action_r) -> msg => {
                    self.handle_message(msg);
                },
            }
        }
    }

    fn start_watch_thread(w: PathBuf, s: FeedbackSender, repaint_signal: Arc<dyn RepaintSignal>) {
        s.send(FeedbackMessage::WatchedChanged).unwrap();

        thread::spawn(move || {
            let watch_opt = WatchOptions {
                file: w,
                debounce_time: Duration::from_secs(1),
            };

            Watcher::try_from(watch_opt)
                .unwrap()
                .watch(&mut || {
                    repaint_signal.request_repaint();
                    s.send(FeedbackMessage::WatchedChanged).unwrap();
                })
                .unwrap();
        });
    }

    fn run_once(opt: &Option<GuiOpt>, s: FeedbackSender) -> Result<(), Error> {
        let opt = opt.as_ref().ok_or(Error::NoResult)?;

        let action = Action::try_from(opt)?;
        let result = action.run()?;

        log::info!("Result is ready, sending");
        s.send(FeedbackMessage::Result(result)).unwrap();

        Ok(())
    }
}

impl TryFrom<&GuiOpt> for Action {
    type Error = Error;

    fn try_from(opt: &GuiOpt) -> Result<Self, Error> {
        if let (Some(lang), Some(source), Some(input), Some(output)) =
            (opt.lang, &opt.source, &opt.input, &opt.output)
        {
            let source = Source::new(source.clone());
            let input = TestDataIn::new(input.clone());
            let output = TestDataOut::new(output.clone());
            let a = Action::BuildTestInputOutput(lang, source, input, output);
            return Ok(a);
        }

        if let (Some(exec), Some(trt)) = (&opt.test_runner, opt.test_runner_type) {
            let exec = exec.clone().into();
            let info = TestRunnerInfo { exec, trt };
            let test_runner = make_test_runner(info);
            let a = Action::TestRunner(test_runner?);
            return Ok(a);
        }

        Ok(Action::Nothing)
    }
}
