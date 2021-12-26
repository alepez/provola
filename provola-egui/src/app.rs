use super::{ActionMessage, ActionSender, FeedbackMessage, FeedbackReceiver};
use crate::central_panel;
use crossbeam_channel::select;
use eframe::{egui, epi};
use provola_core::test_runners::TestRunnerOpt;
use provola_core::{Language, Source, TestDataIn, TestDataOut, TestResult};
use provola_testrunners::TestRunnerInfo;
use std::path::PathBuf;
use std::time::Duration;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum GuiAction {
    BuildTestInputOutput(Language, Source, TestDataIn, TestDataOut),
    TestRunner(TestRunnerInfo, TestRunnerOpt),
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
pub struct GuiOpt {
    pub watch: Option<PathBuf>,
    pub action: Option<GuiAction>,
}

#[derive(Default)]
pub struct State {
    last_result: Option<TestResult>,
}

pub struct ProvolaGuiApp {
    config: GuiOpt,
    state: State,
    s: ActionSender,
    r: FeedbackReceiver,
}

impl ProvolaGuiApp {
    /// Try to resume previous app state
    fn resume_config(&mut self, storage: Option<&dyn epi::Storage>) {
        let stored_config: Option<GuiOpt> = storage.and_then(|s| epi::get_value(s, epi::APP_KEY));

        if let Some(stored_config) = stored_config {
            self.config = stored_config;
        }
    }

    fn send(&mut self, msg: ActionMessage) {
        self.s.send(msg).unwrap();
    }

    fn handle_message(&mut self, msg: FeedbackMessage) {
        let state = &mut self.state;

        match msg {
            FeedbackMessage::Result(new_result) => {
                state.last_result = Some(new_result);
            }
            FeedbackMessage::WatchedChanged => {
                self.action_run_all();
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

    fn action_setup(&mut self, frame: &mut epi::Frame<'_>) {
        // This message is needed to setup the working thread, so it knows
        // how app is configured and how to request a UI repaint.
        let setup = super::Setup {
            opt: self.config.clone(),
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
        });

        // Central panel for test results
        CentralPanel::default().show(ctx, |ui| {
            central_panel::show(ui, &self.state.last_result);
        });
    }
}

impl ProvolaGuiApp {
    pub(crate) fn new(config: GuiOpt, s: ActionSender, r: FeedbackReceiver) -> Self {
        let state = State::default();
        Self {
            config,
            state,
            s,
            r,
        }
    }
}
