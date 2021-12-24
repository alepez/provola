use std::path::Path;
use std::process::Command;

use crate::actions::Source;
use crate::errors::Error;
use crate::Executable;

fn gen_command(exec: &Path, source: &Source) -> Command {
    let mut cmd = Command::new("stack");
    cmd.arg("ghc").arg("--").arg(&source.0).arg("-o").arg(exec);
    cmd
}

pub(crate) fn build(source: &Source) -> Result<Executable, Error> {
    super::build(source, gen_command)
}
