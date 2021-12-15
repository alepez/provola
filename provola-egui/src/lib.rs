mod app;

pub use app::Config as GuiOpt;
use app::ProvolaGuiApp;
use crossbeam_channel::{bounded, select, Receiver, Sender};
use provola_core::{
    Action, Error, Language, Source, TestDataIn, TestDataOut, WatchOptions, Watcher,
};
use provola_testrunners::{make_test_runner, TestRunnerInfo, TestRunnerType};
use std::thread;

enum Message {
    Setup(GuiOpt),
    Result(provola_core::TestResult),
    RunAll,
}

type MessageSender = Sender<Message>;
type MessageReceiver = Receiver<Message>;

pub fn run(opt: GuiOpt) -> Result<(), Error> {
    // Create a channel between working thread and main event loop:
    let (action_s, action_r) = bounded(1000);
    let (feedback_s, feedback_r) = bounded(1000);

    start_working_thread(feedback_s, action_r);

    let app = ProvolaGuiApp::new(opt, action_s, feedback_r);
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);
}

fn start_working_thread(s: MessageSender, r: MessageReceiver) {
    log::debug!("start_working_thread");
    thread::spawn(move || {
        log::debug!("start_working_thread, spawned");
        // TODO Handle error
        run_forever(s, r).unwrap();
    });
}

fn run_forever(mut s: MessageSender, mut r: MessageReceiver) -> Result<(), Error> {
    log::debug!("run_forever");

    let mut opt = Option::<GuiOpt>::default();

    loop {
        select! {
            recv(r) -> msg => {
                match msg {
                    Ok(Message::Setup(new_opt)) => {
                        log::info!("Setup!");
                        opt = Some(new_opt);
                    }
                    Ok(Message::RunAll) => {
                        log::debug!("Receive Message::RunAll");
                        // TODO Give a feedback if run_once return an error
                        run_once(&opt, s.clone()).ok();
                    }
                    _ => {
                    }
                }
            },
        }
    }

    // // TODO Handle error
    // run_once(&opt, &mut s).unwrap();

    // let watch_opt = WatchOptions {
    //     // TODO Handle error
    //     file: opt.watch.as_ref().unwrap().into(),
    //     debounce_time: Duration::from_secs(1),
    // };

    // Watcher::try_from(watch_opt)?.watch(&mut || {
    //     // TODO Handle error
    //     run_once(&opt, &mut s).unwrap();
    // })?;

    Ok(())
}

fn run_once(opt: &Option<GuiOpt>, s: MessageSender) -> Result<(), Error> {
    let opt = opt.as_ref().ok_or(Error::NoResult)?;

    let action = Action::try_from(opt)?;
    let result = action.run()?;

    log::info!("Result is ready, sending");
    s.send(Message::Result(result)).unwrap();

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
