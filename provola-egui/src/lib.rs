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
    // Server and GUI are communicating throug channels
    let (action_s, action_r) = bounded(1000);
    let (feedback_s, feedback_r) = bounded(1000);

    let mut server = Server {
        opt: None,
        repaint_signal: None,
        action_r,
        feedback_s,
    };

    thread::spawn(move || {
        server.run();
    });

    // Create the GUI application
    let app = ProvolaGuiApp::new(opt, action_s, feedback_r);
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options)
}

impl Server {
    fn handle_message(&mut self, msg: ActionMessage) {
        match msg {
            ActionMessage::Setup(setup) => {
                let new_opt = setup.opt;
                let new_repaint_signal = setup.repaint_signal;

                // This must be done before starting watch thread
                self.repaint_signal = Some(new_repaint_signal);

                if let Some(file_to_watch) = &new_opt.watch {
                    // FIXME Make this thread stoppable (when file_to_watch changes)
                    self.start_watch_thread(file_to_watch.clone());
                }

                self.opt = Some(new_opt);
            }
            ActionMessage::RunAll => {
                // TODO Give a feedback if run_once return an error
                self.run_once().ok();
                if let Some(repaint_signal) = &self.repaint_signal {
                    repaint_signal.request_repaint();
                }
            }
        }
    }

    fn run(&mut self) {
        loop {
            select! {
                recv(self.action_r) -> msg => {
                    match msg {
                        Ok(msg) => self.handle_message(msg),
                        Err(_) => return,
                    }
                },
            }
        }
    }

    fn start_watch_thread(&self, w: PathBuf) {
        let feedback_s = self.feedback_s.clone();
        let repaint_signal = self.repaint_signal.clone().unwrap();

        feedback_s.send(FeedbackMessage::WatchedChanged).unwrap();

        thread::spawn(move || {
            let watch_opt = WatchOptions {
                file: w,
                debounce_time: Duration::from_secs(1),
            };

            Watcher::try_from(watch_opt)
                .unwrap()
                .watch(&mut || {
                    repaint_signal.request_repaint();
                    feedback_s.send(FeedbackMessage::WatchedChanged).unwrap();
                })
                .unwrap();
        });
    }

    fn run_once(&self) -> Result<(), Error> {
        let opt = self.opt.as_ref().ok_or(Error::NoResult)?;

        let action = Action::try_from(opt)?;
        let result = action.run()?;

        self.feedback_s
            .send(FeedbackMessage::Result(result))
            .unwrap();

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
