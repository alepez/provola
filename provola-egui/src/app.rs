use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Default)]
struct Config {
    // Persistent configuration
}

pub struct ProvolaGuiApp {
    config: Config,
    receiver: futures::channel::mpsc::Receiver<String>,
}

impl epi::App for ProvolaGuiApp {
    fn name(&self) -> &str {
        "Provola"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            self.config = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, &self.config);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
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
        egui::SidePanel::left("side_panel").show(ctx, |_ui| {
            // TODO
        });

        // Central panel for test results
        egui::CentralPanel::default().show(ctx, |_ui| {
            // TODO
        });
    }
}

impl ProvolaGuiApp {
    pub(crate) fn new(receiver: futures::channel::mpsc::Receiver<String>) -> Self {
        Self {
            config: Config::default(),
            receiver,
        }
    }
}
