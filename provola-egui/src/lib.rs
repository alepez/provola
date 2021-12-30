mod app;
mod central_panel;
mod server;

pub use crate::app::ProvolaGuiApp;
use crate::server::Server;
use crossbeam_channel::{bounded, Receiver, Sender};
use eframe::epi::backend::RepaintSignal;
use provola_core::test_runners::TestRunnerOpt;
use provola_core::*;
use provola_core::{Action, AvailableTests, Error, TestResult};
use provola_testrunners::make_test_runner;
use provola_testrunners::TestRunnerInfo;
use std::path::PathBuf;
use std::{sync::Arc, thread};

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

    let feedback_s = FeedbackSender {
        sender: feedback_s,
        repaint_signal: None,
    };

    let mut server = Server::new(action_r, feedback_s);

    thread::spawn(move || {
        server.run();
    });

    // Create the GUI application
    let app = ProvolaGuiApp::new(opt, action_s, feedback_r);
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub enum ActionConfig {
    BuildTestInputOutput(Language, Source, TestDataIn, TestDataOut),
    TestRunner(TestRunnerInfo, TestRunnerOpt),
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct GuiConfig {
    pub watch_path: Option<PathBuf>,
    pub watch: bool,
    pub action: Option<ActionConfig>,
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
