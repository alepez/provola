use crate::{Executable, TestDataIn, TestDataOut, TestResult};
use std::convert::TryInto;

pub fn test(
    executable: &Executable,
    input: &TestDataIn,
    output: &TestDataOut,
) -> Result<TestResult, Box<dyn std::error::Error>> {
    use subprocess::*;

    let path = executable.path();

    log::debug!("Executing {:?}", path);

    let mut p = Popen::create(
        &[path],
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
    let eq = expected_output == actual_output;

    let result = if eq {
        TestResult::Pass
    } else {
        let msg = format!("Expected\n\n{}\n\nActual\n\n{}", expected_output, actual_output);
        TestResult::Fail(msg.into())
    };

    Ok(result)
}
