use crate::test_runners::{TestRunner, TestRunnerOpt};
use crate::{Error, Executable, Language, TestResult};
use std::{convert::TryFrom, io::Read, path::PathBuf};

#[derive(Debug)]
pub struct Source(pub PathBuf);

impl Source {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

#[derive(Debug)]
pub struct TestDataIn(PathBuf);

impl TestDataIn {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl TryFrom<&TestDataIn> for std::fs::File {
    type Error = Error;

    fn try_from(x: &TestDataIn) -> Result<Self, Self::Error> {
        std::fs::File::open(&x.0).map_err(Error::InvalidInputData)
    }
}

#[derive(Debug)]
pub struct TestDataOut(PathBuf);

impl TestDataOut {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl TryFrom<&TestDataOut> for std::fs::File {
    type Error = Error;

    fn try_from(x: &TestDataOut) -> Result<std::fs::File, Self::Error> {
        std::fs::File::open(&x.0).map_err(Error::InvalidOutputData)
    }
}

impl TryFrom<&TestDataOut> for String {
    type Error = Error;

    fn try_from(x: &TestDataOut) -> Result<String, Self::Error> {
        let mut content = String::new();
        let mut file = std::fs::File::try_from(x)?;
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

pub enum Action {
    Nothing,
    BuildTestInputOutput(Language, Source, TestDataIn, TestDataOut),
    TestRunner(Box<dyn TestRunner>, TestRunnerOpt),
}

impl Action {
    pub fn run(&self) -> Result<TestResult, Error> {
        match self {
            Action::Nothing => Err(Error::NoResult),
            Action::BuildTestInputOutput(lang, source, input, output) => {
                let executable = Some(Executable::try_from((*lang, source))?);
                let executable = executable.as_ref().ok_or(Error::NoExecutable)?;
                crate::test::data::test(executable, input, output)
            }

            // FIXME pass options to run()
            Action::TestRunner(runner, _opt) => runner.run(),
        }
    }
}
