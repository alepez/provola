use std::{convert::TryFrom, io::Read, path::PathBuf};

use crate::Language;

#[derive(Debug)]
pub struct Source(pub PathBuf);

impl Source {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

#[derive(Debug)]
pub struct Executable(PathBuf);

#[derive(Debug)]
pub struct TestDataIn(PathBuf);

impl TestDataIn {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl TryFrom<&TestDataIn> for std::fs::File {
    type Error = std::io::Error;

    fn try_from(x: &TestDataIn) -> Result<Self, Self::Error> {
        std::fs::File::open(&x.0)
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
    type Error = std::io::Error;

    fn try_from(x: &TestDataOut) -> Result<std::fs::File, Self::Error> {
        std::fs::File::open(&x.0)
    }
}

impl TryFrom<&TestDataOut> for String {
    type Error = std::io::Error;

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
