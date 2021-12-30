use super::{ActionMessage, ActionSender, FeedbackMessage, FeedbackReceiver, GuiConfig};
use crate::central_panel;
use crossbeam_channel::select;
use eframe::egui::Color32;
use eframe::{egui, epi};
use egui::*;
use provola_core::test_runners::TestRunnerOpt;
use provola_core::*;
use provola_testrunners::TestRunnerInfo;
use std::path::PathBuf;
use std::time::Duration;

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

    fn action_setup(&mut self, frame: &epi::Frame) {
        // This message is needed to setup the working thread, so it knows
        // how app is configured and how to request a UI repaint.
        let frame_data = frame.lock();
        let repaint_signal = frame_data.repaint_signal.clone();
        let setup = super::Setup {
            config: self.config.clone(),
            repaint_signal,
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
        frame: &epi::Frame,
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
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        self.handle_messages();

        let mut new_config = self.config.clone();

        // Top panel contains the main menu
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                menu::menu_button(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                menu::menu_button(ui, "Help", |ui| {
                    ui.add(Hyperlink::from_label_and_url(
                        RichText::new("About this project"),
                        "https://github.com/alepez/provola",
                    ))
                });
            });

            warn_if_debug_build(ui);
        });

        // Side panel for global actions and feedbacks
        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
                result_info(ui, &self.state.last_result);

                if ui.button("Run all").clicked() {
                    self.action_run_all();
                }

                if ui.button("Scan").clicked() {
                    self.action_req_available_tests();
                }

                ui.checkbox(&mut new_config.watch, "Watch");
            });
        });

        // Central panel for test results
        CentralPanel::default().show(ctx, |ui| {
            if let Some(test_result) = &self.state.last_result {
                central_panel::show(ui, Some(test_result));
            } else if let Some(available_tests) = &self.state.available_tests {
                let report = CoreReport::from(available_tests);
                let test_result = TestResult::from(report);
                central_panel::show(ui, Some(&test_result));
            } else {
                central_panel::show(ui, None);
            }
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

fn result_info(ui: &mut Ui, result: &Option<TestResult>) -> Response {
    let text = text_from_result(result);
    let color = color_from_result(result);
    let rich_text = RichText::new(text).color(color);
    let label = Label::new(rich_text);

    ui.add(label)
}

fn color_from_result(result: &Option<TestResult>) -> Color32 {
    match result {
        None => Color32::LIGHT_GRAY,
        Some(TestResult::Pass(_)) => Color32::GREEN,
        Some(TestResult::Fail(_)) => Color32::RED,
    }
}

fn text_from_result(result: &Option<TestResult>) -> &'static str {
    match result {
        None => "-",
        Some(TestResult::Pass(_)) => "PASS",
        Some(TestResult::Fail(_)) => "FAIL",
    }
}
