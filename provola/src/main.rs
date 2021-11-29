use provola_core::*;
use std::{convert::TryInto, path::PathBuf};
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

fn make_build_rust_command(source: &Source) -> std::process::Command {
    use std::process::Command;
    let mut cmd = Command::new("rustc");
    cmd.arg(&source.0).arg("-o").arg("tmp.exe");
    cmd
}

fn build_rust(source: &Source) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = make_build_rust_command(source);
    log::info!("Running {:?}", cmd);
    cmd.output()?;
    Ok(())
}

fn test_input_output(
    input: &TestDataIn,
    output: &TestDataOut,
) -> Result<TestResult, Box<dyn std::error::Error>> {
    use subprocess::*;

    let mut p = Popen::create(
        &["./tmp.exe"],
        PopenConfig {
            stdin: Redirection::File(input.try_into()?),
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let (out, _err) = p.communicate(None)?;

    if let Some(_exit_status) = p.poll() {
        log::debug!("Test done");
    } else {
        log::warn!("Terminate subprocess");
        p.terminate()?;
    }

    let actual_output = out.unwrap();
    let expected_output: String = output.try_into()?;

    let result = if expected_output == actual_output {
        TestResult::Pass
    } else {
        TestResult::Fail
    };

    log::debug!("{:?}", result);

    Ok(result)
}

fn run_actions(actions: &Actions) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Run actions");
    for action in actions.0.iter() {
        match action {
            Action::Build(Language::Rust, source) => {
                build_rust(source)?;
            }
            Action::TestInputOutput(input, output) => {
                test_input_output(input, output)?;
            }
            _ => todo!(),
        }
    }

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
