use super::{ActionMessage, ActionSender, FeedbackMessage, FeedbackReceiver};
use crossbeam_channel::select;
use eframe::{egui, epi};
use merge::Merge;
use provola_core::{Language, TestResult};
use provola_testrunners::TestRunnerType;
use std::path::PathBuf;
use std::time::Duration;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Default, Clone, Debug, Merge)]
pub struct Config {
    // Persistent configuration
    pub watch: Option<PathBuf>,
    pub input: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub lang: Option<Language>,
    pub source: Option<PathBuf>,
    pub test_runner: Option<PathBuf>,
    pub test_runner_type: Option<TestRunnerType>,
}

#[derive(Default)]
pub struct State {
    last_result: Option<TestResult>,
}

pub struct ProvolaGuiApp {
    config: Config,
    state: State,
    s: ActionSender,
    r: FeedbackReceiver,
}

impl ProvolaGuiApp {
    fn resume_config(&mut self, _storage: Option<&dyn epi::Storage>) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            let stored_config: Config = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
            // If options have been passed as cli arguments, we override stored
            // option with the new ones.
            self.config.merge(stored_config);
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
    #[cfg(feature = "persistence")]
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
        CentralPanel::default().show(ctx, |_ui| {
            // TODO
        });
    }
}

impl ProvolaGuiApp {
    pub(crate) fn new(config: Config, s: ActionSender, r: FeedbackReceiver) -> Self {
        let state = State::default();
        Self {
            config,
            state,
            s,
            r,
        }
    }
}
