use clap::{App, IntoApp, Parser};
use clap_generate::{generate, Generator, Shell};
use provola_core::test_runners::{Only, TestRunnerOpt};
use provola_core::*;
use provola_reporters::{ReporterType, DEFAULT_REPORTER_STR};
use provola_testrunners::make_test_runner;
use provola_testrunners::{TestRunnerInfo, TestRunnerType};
use std::convert::TryFrom;
use std::path::PathBuf;

mod cli;

#[derive(Debug, Parser)]
#[clap(name = "provola", about = "provola, the quick tester")]
struct Opt {
    /// Activate debug mode
    #[clap(long)]
    debug: bool,
    /// Activate gui mode
    #[clap(long)]
    gui: bool,
    /// If provided, outputs the completion file for given shell
    #[clap(long, arg_enum)]
    shell_compl: Option<Shell>,
    /// Watch files or directories for changes
    #[clap(short, long, parse(from_os_str))]
    watch: Option<PathBuf>,
    /// Prevent automatic watch
    #[clap(long, conflicts_with = "watch")]
    no_watch: bool,
    /// Input file to be used for data test
    #[clap(short, long, parse(from_os_str), conflicts_with = "test-runner")]
    input: Option<PathBuf>,
    /// Expected output to be used for data test
    #[clap(short, long, parse(from_os_str), conflicts_with = "test-runner")]
    output: Option<PathBuf>,
    /// Language of the source code. If not provided, it is automatically detected
    #[clap(short, long, conflicts_with = "test-runner")]
    lang: Option<Language>,
    /// Source code file
    #[clap(short, long, conflicts_with = "test-runner")]
    source: Option<PathBuf>,
    /// Execute a test runner
    #[clap(short = 't', requires_all = &["test-runner-type"])]
    test_runner: Option<PathBuf>,
    /// Select test runner type
    #[clap(short = 'T', requires_all = &["test-runner"])]
    test_runner_type: Option<TestRunnerType>,
    /// List available tests
    #[clap(long, requires_all = &["test-runner"])]
    list: bool,
    /// Select reporter type
    #[clap(short = 'R', default_value = & DEFAULT_REPORTER_STR)]
    reporter: ReporterType,
    /// Specify which test number to run. See --list for available tests
    #[clap(long, requires_all = &["test-runner"])]
    only: Option<usize>,
}

impl Opt {
    fn lang_or_guess(&self) -> Option<Language> {
        let source = self.source.as_ref();
        self.lang
            .or_else(|| source.and_then(|x| Language::from_source(x)))
    }

    fn infer_options(mut self) -> Self {
        self.lang = self.lang_or_guess();

        if let Some(test_runner) = &self.test_runner {
            if !self.no_watch {
                self.watch = Some(test_runner.clone());
            }
        }

        self
    }

    fn reporter(&self) -> Result<Box<dyn Reporter>, Error> {
        provola_reporters::make_reporter(self.reporter)
    }
}

impl From<&Opt> for TestRunnerOpt {
    fn from(opt: &Opt) -> Self {
        match opt.only {
            None => TestRunnerOpt { only: Only::All },
            Some(id) => TestRunnerOpt {
                only: Only::SingleByIndex(id),
            },
        }
    }
}

#[cfg(feature = "egui")]
impl TryFrom<&Opt> for provola_egui::ActionConfig {
    type Error = Error;

    fn try_from(opt: &Opt) -> Result<Self, Error> {
        if let (Some(lang), Some(source), Some(input), Some(output)) =
            (opt.lang, &opt.source, &opt.input, &opt.output)
        {
            let source = Source::new(source.clone());
            let input = TestDataIn::new(input.clone());
            let output = TestDataOut::new(output.clone());
            let a = Self::BuildTestInputOutput(lang, source, input, output);
            return Ok(a);
        }

        if let (Some(exec), Some(trt)) = (&opt.test_runner, opt.test_runner_type) {
            let exec = exec.clone().into();
            let info = TestRunnerInfo { exec, trt };
            let a = Self::TestRunner(info, opt.into());
            return Ok(a);
        }

        Err(Error::NothingToDo)
    }
}

// TODO Create from ActionConfig
impl TryFrom<&Opt> for Action {
    type Error = Error;

    fn try_from(opt: &Opt) -> Result<Self, Error> {
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
            let a = Action::TestRunner(test_runner?, opt.into());
            return Ok(a);
        }

        Err(Error::NothingToDo)
    }
}

#[cfg(feature = "egui")]
impl TryFrom<Opt> for provola_egui::GuiConfig {
    type Error = Error;

    fn try_from(opt: Opt) -> Result<Self, Error> {
        let watch_path = opt.watch.clone();
        let watch = !opt.no_watch;
        Ok(Self {
            watch_path,
            watch,
            action: (&opt).try_into().ok(),
        })
    }
}

fn print_completions<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut std::io::stdout());
}

#[cfg(feature = "egui")]
fn run_gui(opt: Opt) -> Result<(), Error> {
    let opt = provola_egui::GuiConfig::try_from(opt)?;
    if let Err(e) = provola_egui::run(opt) {
        log::error!("{}", e);
    }

    Ok(())
}

#[cfg(not(feature = "egui"))]
fn run_gui(_opt: Opt) -> Result<(), Error> {
    Err(Error::GuiNotAvailable)
}

fn main() {
    env_logger::init();

    let opt = Opt::parse().infer_options();

    if let Some(shell_compl) = opt.shell_compl {
        let mut app = Opt::into_app();
        print_completions(shell_compl, &mut app);
        return;
    }

    if opt.gui {
        return run_gui(opt).unwrap();
    }

    if let Err(e) = cli::run(&opt) {
        log::error!("{}", e);
    }
}
