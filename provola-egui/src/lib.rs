#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use provola_core::*;
use provola_testrunners::TestRunnerType;
use std::path::PathBuf;
mod app;
use app::ProvolaGuiApp;

#[derive(Clone)]
pub struct GuiOpt {
    pub watch: Option<PathBuf>,
    pub input: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub lang: Option<Language>,
    pub source: Option<PathBuf>,
    pub test_runner: Option<PathBuf>,
    pub test_runner_type: Option<TestRunnerType>,
}

pub fn run(_opt: GuiOpt) -> Result<(), Error> {
    let app = ProvolaGuiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
