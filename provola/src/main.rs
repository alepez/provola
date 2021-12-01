use provola_core::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "provola", about = "provola, the quick tester")]
struct Opt {
    /// Activate debug mode
    #[structopt(long)]
    debug: bool,
    /// Watch files or directories for changes
    #[structopt(short, long, parse(from_os_str))]
    watch: Option<PathBuf>,
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
    /// Expected output
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    /// Language
    #[structopt(short, long)]
    lang: Option<Language>,
    /// Source code
    #[structopt(short, long)]
    source: Option<PathBuf>,
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

        Actions(actions)
    }
}

fn watch(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    todo!();
}

fn run_once(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    let actions = Actions::from(&opt);
    let result = actions.run()?;
    let reporter = SimpleReporter::new();

    reporter.report(result);

    Ok(())
}

fn run(opt: Opt) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(_watch_files) = &opt.watch {
        watch(opt)
    } else {
        run_once(opt)
    }
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    if let Err(e) = run(opt) {
        log::error!("{}", e);
        Opt::clap().print_long_help().unwrap();
    }
}
