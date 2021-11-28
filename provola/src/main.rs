use provola_core::{Action, Actions, Language};
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

        if opt.source.is_some() {
            actions.push(Action::Build);
        }

        if opt.input.is_some() && opt.output.is_some() {
            actions.push(Action::TestInputOutput);
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
