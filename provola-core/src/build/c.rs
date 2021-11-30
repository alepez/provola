use std::path::Path;
use std::process::Command;

use crate::Executable;
use crate::actions::Source;
use crate::errors::Error;

fn gen_command(exec: &Path, source: &Source) -> Command {
    let mut cmd = Command::new("gcc");
    cmd.arg(&source.0).arg("-o").arg(exec);
    cmd
}

pub(crate) fn build(source: &Source) -> Result<Executable, Error> {
    super::build(source, gen_command)
}
