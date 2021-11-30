use std::path::Path;
use std::process::Command;

use crate::actions::Source;
use crate::errors::Error;
use crate::Executable;

fn gen_command(_exec: &Path, _ource: &Source) -> Command {
    todo!()
}

pub(crate) fn build(source: &Source) -> Result<Executable, Error> {
    super::build(source, gen_command)
}
