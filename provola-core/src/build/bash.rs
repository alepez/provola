use crate::actions::Source;
use crate::errors::Error;
use crate::Executable;

pub(crate) fn build(source: &Source) -> Result<Executable, Error> {
    super::common::interpret(source, "bash")
}
