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

impl epi::App for ProvolaGuiApp {
    fn name(&self) -> &str {
        "Provola"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            let stored_config: Config = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
            // If options have been passed as cli arguments, we override stored
            // option with the new ones.
            self.config.merge(stored_config);
        }

        let repaint_signal = frame.repaint_signal();

        let setup = super::Setup {
            opt: self.config.clone(),
            repaint_signal,
        };

        self.s.send(ActionMessage::Setup(setup)).unwrap();
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, &self.config);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let state = &mut self.state;

        select! {
            recv(self.r) -> msg => {
                match msg {
                    Ok(FeedbackMessage::Result(new_result)) => {
                        log::info!("Test result is ready");
                        state.last_result = Some(new_result);
                    }
                    Ok(FeedbackMessage::WatchedChanged) => {
                        log::info!("Watched file has changed");
                        state.last_result = None;
                        self.s.send(ActionMessage::RunAll).unwrap();
                    }
                    _ => {}
                }
            },
            default(Duration::from_millis(1)) => {}
        }

        // Top panel contains the main menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                egui::menu::menu(ui, "Help", |ui| {
                    egui::warn_if_debug_build(ui);
                    ui.add(
                        egui::Hyperlink::new("https://github.com/alepez/provola")
                            .text("About this project")
                            .small(),
                    )
                });
            });
        });

        // Side panel for global actions and feedbacks
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            let result_str = match state.last_result {
                None => "-",
                Some(TestResult::Pass(_)) => "PASS",
                Some(TestResult::Fail(_)) => "FAIL",
            };

            ui.strong(result_str);

            if ui.button("Run all").clicked() {
                log::debug!("Send Message::RunAll");
                state.last_result = None;
                self.s.send(ActionMessage::RunAll).unwrap();
            }
        });

        // Central panel for test results
        egui::CentralPanel::default().show(ctx, |_ui| {
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
