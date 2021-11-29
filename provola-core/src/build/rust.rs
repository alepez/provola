use std::path::PathBuf;
use std::process::Command;

use crate::actions::Source;
use crate::errors::Error;

fn gen_command(exec: &PathBuf, source: &Source) -> Command {
    let mut cmd = Command::new("rustc");
    cmd.arg(&source.0).arg("-o").arg(exec);
    cmd
}

pub(crate) fn build(source: &Source) -> Result<PathBuf, Error> {
    super::common::build(source, gen_command)
}
