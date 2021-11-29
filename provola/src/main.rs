use provola_core::*;
use std::{convert::TryFrom, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "provola", about = "provola, your personal tester")]
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

        if let (Some(lang), Some(source)) = (opt.lang, &opt.source) {
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

fn run_actions(actions: &Actions) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Run actions");

    let mut executable: Option<Executable> = Default::default();
    let mut result: Option<TestResult> = Default::default();

    for action in actions.0.iter() {
        match action {
            Action::Build(lang, source) => {
                executable = Some(Executable::try_from((*lang, source))?);
            }
            Action::TestInputOutput(input, output) => {
                let executable = executable.as_ref().expect("No executable");
                use provola_core::test::data::test;
                result = Some(test(&executable, input, output)?);
            }
        }
    }

    log::info!("{:?}", result);

    Ok(())
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();
    let actions = Actions::from(&opt);
    log::info!("{:?}", opt);
    log::info!("{:?}", actions);

    run_actions(&actions).unwrap();
}
