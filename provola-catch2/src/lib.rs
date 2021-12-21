use provola_core::{Error, Executable, Report};
use std::time::Duration;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

mod report;

fn add_arguments(mut argv: Vec<String>) -> Vec<String> {
    argv.push("-r".into());
    argv.push("junit".into());
    argv
}

fn run_exec(executable: &Executable) -> Result<Report, Error> {
    let argv = add_arguments(executable.into());

    let mut p = Popen::create(
        &argv,
        PopenConfig {
            stdin: Redirection::None,
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let (out, _err) = p.communicate(None)?;

    // TODO Timeout from configuration
    let timeout = Duration::from_secs(3600);

    if let Some(_exit_status) = p.wait_timeout(timeout)? {
        log::debug!("Test done");
    } else {
        log::warn!("Terminate subprocess");
        p.terminate()?;
    }

    if let Some(out) = out {
        let rep: report::Report =
            serde_xml_rs::from_str(&out).map_err(|e| Error::ReportParseError(Box::new(e)))?;
        let core_rep = Report::from(rep);
        Ok(core_rep)
    } else {
        Err(Error::ReportUnavailable)
    }
}

pub struct TestRunner {
    executable: Executable,
}

impl From<Executable> for TestRunner {
    fn from(executable: Executable) -> Self {
        Self { executable }
    }
}

impl provola_core::test_runners::TestRunner for TestRunner {
    fn run(&self) -> Result<provola_core::TestResult, provola_core::Error> {
        let report = run_exec(&self.executable)?;
        let result = report.into();
        Ok(result)
    }
}
