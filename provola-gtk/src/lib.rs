use futures::{channel::mpsc, StreamExt};
use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Label};
use provola_core::Error;
use provola_core::*;
use provola_reporters::{ReporterType, DEFAULT_REPORTER_STR};
use provola_testrunners::make_test_runner;
use provola_testrunners::{TestRunnerInfo, TestRunnerType};
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

const APPLICATION_ID: &'static str = "dev.alepez.provola";

#[derive(Clone)]
pub struct GuiOpt {
    pub watch: Option<PathBuf>,
    pub input: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub lang: Option<Language>,
    pub source: Option<PathBuf>,
    pub test_runner: Option<PathBuf>,
    pub test_runner_type: Option<TestRunnerType>,
    pub reporter: ReporterType,
}

pub fn run(opt: GuiOpt) -> Result<(), Error> {
    let application = gtk::Application::new(Some(APPLICATION_ID), Default::default());

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        let label = Label::new(None);

        window.add(&label);

        // Create a channel between communication thread and main event loop:
        let (sender, receiver) = mpsc::channel(1000);

        spawn_local_handler(label, receiver);

        // TODO Avoid clone
        start_communication_thread(sender, opt.clone());
        window.show_all();
    });

    // We must use run_with_args or GTK will try to parse command line arguments
    let args: [&str; 0] = [];
    application.run_with_args(&args);

    Ok(())
}

/// Spawn channel receive task on the main event loop.
fn spawn_local_handler(label: gtk::Label, mut receiver: mpsc::Receiver<String>) {
    let main_context = glib::MainContext::default();
    let future = async move {
        while let Some(item) = receiver.next().await {
            label.set_text(&item);
        }
    };
    main_context.spawn_local(future);
}

/// Spawn separate thread to handle communication.
fn start_communication_thread(mut sender: mpsc::Sender<String>, opt: GuiOpt) {
    // Note that blocking I/O with threads can be prevented
    // by using asynchronous code, which is often a better
    // choice. For the sake of this example, we showcase the
    // way to use a thread when there is no other option.

    thread::spawn(move || {
        run_forever(opt, sender);
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

impl GuiOpt {
    fn lang_or_guess(&self) -> Option<Language> {
        let source = self.source.as_ref();
        self.lang
            .or_else(|| source.and_then(|x| Language::from_source(x)))
    }

    fn infer_options(&mut self) {
        self.lang = self.lang_or_guess();

        if let Some(test_runner) = &self.test_runner {
            self.watch = Some(test_runner.clone());
        }
    }

    fn reporter(&self) -> Result<Box<dyn Reporter>, Error> {
        provola_reporters::make_reporter(self.reporter)
    }
}
