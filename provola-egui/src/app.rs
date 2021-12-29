use super::{ActionMessage, ActionSender, FeedbackMessage, FeedbackReceiver};
use crate::central_panel;
use crossbeam_channel::select;
use eframe::{egui, epi};
use provola_core::test_runners::TestRunnerOpt;
use provola_core::{AvailableTests, Language, Source, TestDataIn, TestDataOut, TestResult};
use provola_testrunners::TestRunnerInfo;
use std::path::PathBuf;
use std::time::Duration;

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

#[derive(Default)]
pub struct State {
    last_result: Option<TestResult>,
    available_tests: Option<AvailableTests>,
}

pub struct ProvolaGuiApp {
    config: GuiConfig,
    state: State,
    s: ActionSender,
    r: FeedbackReceiver,
}

/// Merges current configuration with stored configuration, giving priority to
/// current configuration.
fn merge(config: &mut GuiConfig, stored_config: GuiConfig) {
    if config.watch_path.is_none() {
        config.watch_path = stored_config.watch_path;
    }
    if config.action.is_none() {
        config.action = stored_config.action;
    }
}

impl ProvolaGuiApp {
    /// Try to resume previous app state
    fn resume_config(&mut self, storage: Option<&dyn epi::Storage>) {
        // Get stored configuration
        let stored_config: Option<GuiConfig> =
            storage.and_then(|s| epi::get_value(s, epi::APP_KEY));

        // Merge current configuration with stored configuration
        if let Some(stored_config) = stored_config {
            merge(&mut self.config, stored_config);
        }
    }

    fn send(&mut self, msg: ActionMessage) {
        self.s.send(msg).unwrap();
    }

    fn handle_message(&mut self, msg: FeedbackMessage) {
        let state = &mut self.state;

        match msg {
            FeedbackMessage::AvailableTests(tests) => {
                state.available_tests = Some(tests);
            }
            FeedbackMessage::Result(new_result) => {
                state.last_result = Some(new_result);
            }
            FeedbackMessage::WatchedChanged => {
                // Avoid running all tests when watch is false.
                // This is needed because watching thread may still be active.
                if self.config.watch {
                    self.action_run_all();
                }
            }
            FeedbackMessage::Error(_error) => {
                // TODO Show error
            }
        }
    }

    fn handle_messages(&mut self) {
        select! {
            recv(self.r) -> msg => {
                match msg {
                    Ok(msg) => self.handle_message(msg),
                    Err(_) => return,
                }
            },
            default(Duration::from_millis(1)) => {}
        }
    }

    fn action_run_all(&mut self) {
        self.state.last_result = None;
        self.send(ActionMessage::RunAll);
    }

    fn action_req_available_tests(&mut self) {
        self.state.last_result = None;
        self.state.available_tests = None;
        self.send(ActionMessage::ReqAvailableTests);
    }

    fn action_setup(&mut self, frame: &mut epi::Frame<'_>) {
        // This message is needed to setup the working thread, so it knows
        // how app is configured and how to request a UI repaint.
        let setup = super::Setup {
            config: self.config.clone(),
            repaint_signal: frame.repaint_signal(),
        };

        self.send(ActionMessage::Setup(setup));
    }
}

impl epi::App for ProvolaGuiApp {
    fn name(&self) -> &str {
        "Provola"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        self.resume_config(storage);
        self.action_setup(frame);
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, &self.config);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        use egui::*;

        self.handle_messages();

        let mut new_config = self.config.clone();

        // Top panel contains the main menu
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                menu::menu(ui, "Help", |ui| {
                    warn_if_debug_build(ui);
                    ui.add(
                        Hyperlink::new("https://github.com/alepez/provola")
                            .text("About this project")
                            .small(),
                    )
                });
            });
        });

        // Side panel for global actions and feedbacks
        SidePanel::left("side_panel").show(ctx, |ui| {
            let result_str = match self.state.last_result {
                None => "-",
                Some(TestResult::Pass(_)) => "PASS",
                Some(TestResult::Fail(_)) => "FAIL",
            };

            ui.strong(result_str);

            if ui.button("Run all").clicked() {
                self.action_run_all();
            }

            if ui.button("Scan").clicked() {
                self.action_req_available_tests();
            }

            ui.checkbox(&mut new_config.watch, "Watch");
        });

        // Central panel for test results
        CentralPanel::default().show(ctx, |ui| {
            central_panel::show(ui, &self.state.last_result, &self.state.available_tests);
        });

        if new_config != self.config {
            self.send(ActionMessage::UpdateConfig(self.config.clone()));
            self.config = new_config;
        }
    }
}

impl ProvolaGuiApp {
    pub(crate) fn new(config: GuiConfig, s: ActionSender, r: FeedbackReceiver) -> Self {
        let state = State::default();
        Self {
            config,
            state,
            s,
            r,
        }
    }
}
