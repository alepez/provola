use provola_core::lang::Language;
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
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
