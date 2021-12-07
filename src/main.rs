use clap::{App, IntoApp, Parser};
use clap_generate::{generate, Generator, Shell};
use provola_core::test_runners::TestRunnerType;
use provola_core::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[clap(name = "provola", about = "provola, the quick tester")]
struct Opt {
    /// Activate debug mode
    #[clap(long)]
    debug: bool,
    /// Watch files or directories for changes
    #[clap(short, long, parse(from_os_str))]
    watch: Option<PathBuf>,
    /// Input file
    #[clap(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
    /// Expected output
    #[clap(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// Language
    #[clap(short, long)]
    lang: Option<Language>,
    /// Source code
    #[clap(short, long)]
    source: Option<PathBuf>,
    /// If provided, outputs the completion file for given shell
    #[clap(long = "generate", arg_enum)]
    generator: Option<Shell>,
    /// Execute a test runner
    #[clap(short = 't')]
    test_runner: Option<PathBuf>,
    /// Select test runner type
    #[clap(short = 'T')]
    test_runner_type: Option<TestRunnerType>,
}

impl From<&Opt> for Actions {
    fn from(opt: &Opt) -> Self {
        let mut actions = Vec::new();

        let lang = opt
            .lang
            .or_else(|| opt.source.as_ref().and_then(|x| Language::from_source(x)));

        if let (Some(lang), Some(source)) = (lang, &opt.source) {
            let source = Source::new(source.clone());
            actions.push(Action::Build(lang, source));
        }

        if let (Some(input), Some(output)) = (&opt.input, &opt.output) {
            let input = TestDataIn::new(input.clone());
            let output = TestDataOut::new(output.clone());
            actions.push(Action::TestInputOutput(input, output));
        }

        if let (Some(exec), Some(trt)) = (&opt.test_runner, opt.test_runner_type) {
            let exec = exec.clone().into();
            actions.push(Action::TestRunner(exec, trt));
        }

        Actions(actions)
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
        .unwrap();

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
    let actions = Actions::from(opt);

    if !actions.is_valid() {
        return Err(Error::NothingToDo.into());
    }

    let result = actions.run()?;
    let reporter = SimpleReporter::new();

    reporter.report(result);

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

    let opt = Opt::parse();

    if let Some(generator) = opt.generator {
        let mut app = Opt::into_app();
        print_completions(generator, &mut app);
        return;
    }

    if let Err(e) = run(&opt) {
        log::error!("{}", e);
    }
}
