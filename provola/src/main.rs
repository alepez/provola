use provola_core::*;
use std::path::PathBuf;
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

fn main() {
    let opt = Opt::from_args();
    let actions = Actions::from(&opt);
    println!("{:?}", opt);
    println!("{:?}", actions);
}
