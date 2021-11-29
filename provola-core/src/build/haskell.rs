use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use crate::actions::Source;
use crate::errors::Error;

fn gen_command(exec: &PathBuf, source: &Source) -> Command {
    let mut cmd = Command::new("stack");
    cmd.arg("ghc")
        .arg("--")
        .arg(&source.0)
        .arg("-o")
        .arg(exec);
    cmd
}

pub(crate) fn build(source: &Source) -> Result<PathBuf, Error> {
    let exec = PathBuf::from("./tmp.exe");
    let mut cmd = gen_command(&exec, source);
    log::info!("Running {:?}", cmd);
    let Output { status, stderr, .. } = cmd.output()?;

    let stderr = String::from_utf8(stderr).unwrap_or_default();

    if status.success() {
        Ok(exec)
    } else {
        let msg = format!("Cannot build executable with command {:?}: {}", cmd, stderr);
        log::error!("{}", &msg);
        Err(Error::BuildFailed(msg))
    }
}
