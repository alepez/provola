use clap::{App, IntoApp, Parser};
use clap_generate::{generate, Generator, Shell};
use provola_core::*;
use provola_reporters::{ReporterType, DEFAULT_REPORTER_STR};
use provola_testrunners::make_test_runner;
use provola_testrunners::{TestRunnerInfo, TestRunnerType};
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[clap(name = "provola", about = "provola, the quick tester")]
struct Opt {
    /// Activate debug mode
    #[clap(long)]
    debug: bool,
    /// If provided, outputs the completion file for given shell
    #[clap(long, arg_enum)]
    shell_compl: Option<Shell>,
    /// Watch files or directories for changes
    #[clap(short, long, parse(from_os_str))]
    watch: Option<PathBuf>,
    /// Input file to be used for data test
    #[clap(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
    /// Expected output to be used for data test
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// Language of the source code. If not provided, it is automatically detected
    #[clap(short, long)]
    lang: Option<Language>,
    /// Source code file
    #[clap(short, long)]
    source: Option<PathBuf>,
    /// Execute a test runner
    #[clap(short = 't')]
    test_runner: Option<PathBuf>,
    /// Select test runner type
    #[clap(short = 'T')]
    test_runner_type: Option<TestRunnerType>,
    #[clap(short = 'R', default_value = &DEFAULT_REPORTER_STR)]
    reporter: ReporterType,
}

impl Opt {
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
            let a = Action::TestRunner(test_runner?);
            return Ok(a);
        }

        Ok(Action::Nothing)
    }
}

fn watch(opt: &Opt, watch_files: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use notify::{watcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let (tx, rx) = channel();

    let debounce_time = Duration::from_secs(1);
    let mut watcher = watcher(tx, debounce_time).unwrap();

    watcher
        .watch(&watch_files, RecursiveMode::Recursive)
        .map_err(|e| Error::CannotWatch(e.to_string()))?;

    loop {
        match rx.recv() {
            Ok(e) => {
                log::trace!("{:?}", e);
                run_once(opt).ok(); // TODO Print error and continue
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
}

fn run_once(opt: &Opt) -> Result<(), Box<dyn std::error::Error>> {
    let action = Action::try_from(opt)?;
    let result = action.run()?;
    let reporter = opt.reporter()?;

    reporter.report(result)?;

    Ok(())
}

fn run(opt: &Opt) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(watch_files) = &opt.watch {
        run_once(opt).ok(); // TODO Print error and continue
        watch(opt, watch_files)
    } else {
        run_once(opt)
    }
}

fn print_completions<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut std::io::stdout());
}

fn main() {
    env_logger::init();

    let mut opt = Opt::parse();
    opt.infer_options();

    if let Some(shell_compl) = opt.shell_compl {
        let mut app = Opt::into_app();
        print_completions(shell_compl, &mut app);
        return;
    }

    if let Err(e) = run(&opt) {
        log::error!("{}", e);
    }
}
