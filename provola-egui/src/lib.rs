mod app;
mod central_panel;

pub use crate::app::{ActionConfig, GuiConfig, ProvolaGuiApp};
use crossbeam_channel::{bounded, select, Receiver, Sender};
use eframe::epi::RepaintSignal;
use provola_core::{Action, AvailableTests, Error, TestResult, WatchOptions, Watcher};
use provola_testrunners::make_test_runner;
use std::{path::PathBuf, sync::Arc, thread, time::Duration};

struct Setup {
    config: GuiConfig,
    repaint_signal: Arc<dyn RepaintSignal>,
}

enum ActionMessage {
    Setup(Setup),
    RunAll,
    UpdateConfig(GuiConfig),
    ReqAvailableTests,
}

enum FeedbackMessage {
    AvailableTests(AvailableTests),
    Result(TestResult),
    WatchedChanged,
    Error(String),
}

type ActionSender = Sender<ActionMessage>;
type ActionReceiver = Receiver<ActionMessage>;
type FeedbackReceiver = Receiver<FeedbackMessage>;

// When the server send a feedback to the gui, the gui must awake.
// To do that, we use repaint_signal to notify the gui whenever a
// feedback is sent.
#[derive(Clone)]
struct FeedbackSender {
    sender: Sender<FeedbackMessage>,
    repaint_signal: Option<Arc<dyn RepaintSignal>>,
}

impl FeedbackSender {
    fn send(&self, msg: FeedbackMessage) {
        self.sender.send(msg).unwrap();
        if let Some(repaint_signal) = &self.repaint_signal {
            repaint_signal.request_repaint();
        }
    }
}

struct Server {
    opt: Option<GuiConfig>,
    action_r: ActionReceiver,
    feedback_s: FeedbackSender,
}

pub fn run(opt: GuiConfig) -> Result<(), Error> {
    // Server and GUI are communicating throug channels
    let (action_s, action_r) = bounded(1000);
    let (feedback_s, feedback_r) = bounded(1000);

    let mut server = Server {
        opt: None,
        action_r,
        feedback_s: FeedbackSender {
            sender: feedback_s,
            repaint_signal: None,
        },
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
                // This must be done before starting watch thread
                self.feedback_s.repaint_signal = Some(setup.repaint_signal);

                if let Some(watch_path) = &setup.config.watch_path {
                    // FIXME Make this thread stoppable (when watch_path changes)
                    // FIXME Start this thread even if watch_path is currently None
                    self.start_watch_thread(watch_path.clone());
                }

                self.opt = Some(setup.config);

                // TODO Give a feedback if get_available_tests return an error
                self.get_available_tests().ok();
            }
            ActionMessage::RunAll => {
                // TODO Give a feedback if run_once return an error
                self.run_once().ok();
            }
            ActionMessage::UpdateConfig(new_config) => {
                log::debug!("Configuration changed");
                self.opt = Some(new_config);
            }
            ActionMessage::ReqAvailableTests => {
                // TODO Give a feedback if get_available_tests return an error
                if let Err(err) = self.get_available_tests() {
                    log::error!("{}", err);
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

        feedback_s.send(FeedbackMessage::WatchedChanged);

        thread::spawn(move || {
            let watch_opt = WatchOptions {
                file: w,
                debounce_time: Duration::from_secs(1),
            };

            // TODO watch must be stopped when file_to_watch changes
            Watcher::try_from(watch_opt)
                .unwrap()
                .watch(&mut || {
                    log::debug!("Watched change detected");
                    feedback_s.send(FeedbackMessage::WatchedChanged);
                })
                .unwrap();
        });
    }

    fn run_once(&self) -> Result<(), Error> {
        let opt = self.opt.as_ref().ok_or(Error::NoResult)?;

        let action = Action::try_from(opt)?;
        let result = action.run()?;

        self.feedback_s.send(FeedbackMessage::Result(result));

        Ok(())
    }

    fn get_available_tests(&self) -> Result<(), Error> {
        let opt = self.opt.as_ref().ok_or(Error::NoResult)?;

        let action = Action::try_from(opt)?;

        let list = match action {
            Action::TestRunner(tr, opt) => Ok(tr.list(&opt)?),
            _ => Err(Error::NothingToDo),
        };

        match list {
            Ok(list) => {
                self.feedback_s.send(FeedbackMessage::AvailableTests(list));
            }
            Err(err) => {
                self.feedback_s
                    .send(FeedbackMessage::Error(err.to_string()));
            }
        };

        Ok(())
    }
}

impl TryFrom<&GuiConfig> for Action {
    type Error = Error;

    fn try_from(opt: &GuiConfig) -> Result<Self, Error> {
        let action_cfg = opt.action.as_ref().ok_or(Error::NothingToDo)?;

        let action = match action_cfg {
            ActionConfig::BuildTestInputOutput(lang, source, input, output) => {
                Action::BuildTestInputOutput(*lang, source.clone(), input.clone(), output.clone())
            }
            ActionConfig::TestRunner(info, opt) => {
                let test_runner = make_test_runner(info.clone());
                Action::TestRunner(test_runner?, opt.clone())
            }
        };

        Ok(action)
    }
}
