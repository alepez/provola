mod app;
mod central_panel;
mod server;

pub use crate::app::{ActionConfig, GuiConfig, ProvolaGuiApp};
use crate::server::Server;
use crossbeam_channel::{bounded, select, Receiver, Sender};
use eframe::epi::backend::RepaintSignal;
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
