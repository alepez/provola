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

#[derive(Debug)]
pub enum Action {
    Build(Language, Source),
    TestInputOutput(TestDataIn, TestDataOut),
}

#[derive(Debug)]
pub struct Actions(pub Vec<Action>);

impl Actions {
    pub fn run(&self) -> Result<TestResult, Error> {
        let mut executable: Option<Executable> = Default::default();
        let mut result: Option<TestResult> = Default::default();

        for action in self.0.iter() {
            match action {
                Action::Build(lang, source) => {
                    executable = Some(Executable::try_from((*lang, source))?);
                }
                Action::TestInputOutput(input, output) => {
                    let executable = executable.as_ref().ok_or(Error::NoExecutable)?;
                    use crate::test::data::test;
                    result = Some(test(executable, input, output)?);
                }
            }
        }

        if let Some(result) = result {
            Ok(result)
        } else {
            Err(Error::NoResult)
        }
    }
}
