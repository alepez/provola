use provola_core::{Error, Executable, Report};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

mod report;

fn add_arguments(mut argv: Vec<String>, report_path: &str) -> Vec<String> {
    argv.push(format!("--gtest_output=json:{}", report_path));
    argv.push("--gtest_color=no".to_string());
    argv
}

fn run_exec(executable: &Executable) -> Result<Report, Error> {
    let report_path = "googletest_report.json";
    let argv = add_arguments(executable.into(), report_path);

    let mut p = Popen::create(
        &argv,
        PopenConfig {
            stdin: Redirection::None,
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let (_out, _err) = p.communicate(None)?;

    // TODO Timeout from configuration
    let timeout = Duration::from_secs(3600);

    if let Some(_exit_status) = p.wait_timeout(timeout)? {
        log::debug!("Test done");
    } else {
        log::warn!("Terminate subprocess");
        p.terminate()?;
    }

    let file = File::open(report_path).unwrap();
    let reader = BufReader::new(file);
    let gtest_rep: report::UnitTest = serde_json::from_reader(reader).unwrap();
    let core_rep = Report::from(gtest_rep);
    Ok(core_rep)
}

pub struct TestRunner {
    executable: Executable,
}

impl From<Executable> for TestRunner {
    fn from(executable: Executable) -> Self {
        TestRunner { executable }
    }
}

impl provola_core::test_runners::TestRunner for TestRunner {
    fn run(&self) -> Result<provola_core::TestResult, provola_core::Error> {
        let report = run_exec(&self.executable)?;
        let result = report.into();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn run_valid_executable() {
        let path = PathBuf::from("./examples/data/build/example");
        let exec = Executable::from(path);
        assert!(run_exec(&exec).is_ok());
    }

    // Ignored because example must be built first
    #[ignore]
    #[test]
    fn from_valid_executable() {
        let path = PathBuf::from("./examples/data/build/example");
        let exec = Executable::from(path);
        let tr = TestRunner::from(exec);
        let tr: Box<dyn provola_core::test_runners::TestRunner> = Box::new(tr);
        assert!(tr.run().is_ok());
    }
}
