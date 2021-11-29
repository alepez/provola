use std::path::PathBuf;
use std::process::Command;

use crate::Executable;
use crate::actions::Source;
use crate::errors::Error;

fn gen_command(exec: &PathBuf, source: &Source) -> Command {
    let mut cmd = Command::new("rustc");
    cmd.arg(&source.0).arg("-o").arg(exec);
    cmd
}

pub(crate) fn build(source: &Source) -> Result<Executable, Error> {
    super::common::build(source, gen_command)
}

#[cfg(test)]
mod test {
    use super::*;

    fn gen_source(s: &str) -> Source {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(s);
        Source::new(path)
    }

    #[test]
    fn build_valid_program() {
        let source = gen_source("examples/data/app_to_be_tested.rs");
        let exec = build(&source);
        assert!(exec.is_ok());
    }

    #[test]
    fn build_non_existent_program() {
        let source = gen_source("examples/data/this_file_does_not_exist.rs");
        let exec = build(&source);
        assert!(exec.is_err());
    }

    #[test]
    fn build_invalid_program() {
        let source = gen_source("examples/data/invalid_program.rs");
        let exec = build(&source);
        assert!(exec.is_err());
    }
}
