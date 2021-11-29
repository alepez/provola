use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use crate::Executable;
use crate::actions::Source;
use crate::errors::Error;

pub(crate) type CommandGenerator = fn(&PathBuf, &Source) -> Command;

pub(crate) fn build(source: &Source, gen_command: CommandGenerator) -> Result<Executable, Error> {
    let exec = PathBuf::from("./tmp.exe");
    let mut cmd = gen_command(&exec, source);
    log::info!("Running {:?}", cmd);
    let Output { status, stderr, .. } = cmd.output()?;

    let stderr = String::from_utf8(stderr).unwrap_or_default();

    if status.success() {
        Ok(exec.into())
    } else {
        let msg = format!("Cannot build executable with command {:?}: {}", cmd, stderr);
        log::error!("{}", &msg);
        Err(Error::BuildFailed(msg))
    }
}

pub(crate) fn interpret(source: &Source, interpreter: impl ToString) -> Result<Executable, Error> {
    Ok(Executable::interpreted(interpreter.to_string(), source))
}
