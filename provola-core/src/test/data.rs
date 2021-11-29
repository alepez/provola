use crate::{result::Reason, Error, Executable, TestDataIn, TestDataOut, TestResult};
use std::{convert::TryInto, time::Duration};

pub fn test(
    executable: &Executable,
    input: &TestDataIn,
    output: &TestDataOut,
) -> Result<TestResult, Error> {
    use subprocess::*;

    // Read from file
    let stdin = Redirection::File(input.try_into()?);

    // Intercept output, so we can compare it later
    let stdout = Redirection::Pipe;

    let argv: Vec<String> = executable.into();

    let mut p = Popen::create(
        &argv,
        PopenConfig {
            stdin,
            stdout,
            ..Default::default()
        },
    )?;

    let (out, _err) = p.communicate(None)?;

    let timeout = Duration::from_secs(5);

    if let Some(_exit_status) = p.wait_timeout(timeout)? {
        log::debug!("Test done");
    } else {
        log::warn!("Terminate subprocess");
        p.terminate()?;
    }

    let actual_output = out.unwrap();
    let expected_output: String = output.try_into()?;
    let eq = expected_output == actual_output;

    let result = if eq {
        TestResult::Pass
    } else {
        let reason = Reason::not_expected(actual_output, expected_output);
        TestResult::Fail(reason)
    };

    Ok(result)
}
