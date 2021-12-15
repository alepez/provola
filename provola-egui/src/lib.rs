#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use std::path::PathBuf;
mod app;
use app::ProvolaGuiApp;
use futures::{channel::mpsc, StreamExt};
use provola_core::*;
use provola_testrunners::{make_test_runner, TestRunnerInfo, TestRunnerType};
use std::thread;
use std::time::Duration;

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

pub fn run(opt: GuiOpt) -> Result<(), Error> {
    // Create a channel between working thread and main event loop:
    let (sender, receiver) = mpsc::channel(1000);

    let app = ProvolaGuiApp::new(receiver);
    let native_options = eframe::NativeOptions::default();

    // FIXME Options can be changed by gui, so this must be shared between threads
    start_working_thread(sender, opt.clone());

    eframe::run_native(Box::new(app), native_options);
}

fn start_working_thread(sender: mpsc::Sender<String>, opt: GuiOpt) {
    thread::spawn(move || {
        // TODO Handle error
        run_forever(opt, sender).unwrap();
    });
}

fn run_forever(opt: GuiOpt, mut sender: mpsc::Sender<String>) -> Result<(), Error> {
    // TODO Handle error
    run_once(&opt, &mut sender).unwrap();

    let watch_opt = WatchOptions {
        // TODO Handle error
        file: opt.watch.as_ref().unwrap().into(),
        debounce_time: Duration::from_secs(1),
    };

    Watcher::try_from(watch_opt)?.watch(&mut || {
        // TODO Handle error
        run_once(&opt, &mut sender).unwrap();
    })?;

    Ok(())
}

fn run_once(opt: &GuiOpt, sender: &mut mpsc::Sender<String>) -> Result<(), Error> {
    let action = Action::try_from(opt)?;
    let result = action.run()?;

    // TODO Use reason
    let data = match result {
        TestResult::Pass(_reason) => "PASS".to_string(),
        TestResult::Fail(_reason) => "FAIL".to_string(),
    };

    match sender.try_send(data) {
        Ok(_) => {}
        Err(err) => {
            if err.is_full() {
                log::warn!("Data is produced too fast for GUI");
            } else if err.is_disconnected() {
                log::warn!("GUI stopped, stopping thread.");
                return Ok(()); // TODO Return an error
            }
        }
    }

    Ok(())
}

impl TryFrom<&GuiOpt> for Action {
    type Error = Error;

    fn try_from(opt: &GuiOpt) -> Result<Self, Error> {
        if let (Some(lang), Some(source), Some(input), Some(output)) =
            (opt.lang, &opt.source, &opt.input, &opt.output)
        {
            let source = Source::new(source.clone());
            let input = TestDataIn::new(input.clone());
            let output = TestDataOut::new(output.clone());
            let a = Action::BuildTestInputOutput(lang, source, input, output);
            return Ok(a);
        }

        if let (Some(exec), Some(trt)) = (&opt.test_runner, opt.test_runner_type) {
            let exec = exec.clone().into();
            let info = TestRunnerInfo { exec, trt };
            let test_runner = make_test_runner(info);
            let a = Action::TestRunner(test_runner?);
            return Ok(a);
        }

        Ok(Action::Nothing)
    }
}
