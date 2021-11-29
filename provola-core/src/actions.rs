use std::{convert::TryInto, io::Read, path::PathBuf};

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

impl TryInto<std::fs::File> for &TestDataIn {
    type Error = std::io::Error;

    fn try_into(self) -> Result<std::fs::File, Self::Error> {
        std::fs::File::open(&self.0)
    }
}

#[derive(Debug)]
pub struct TestDataOut(PathBuf);

impl TestDataOut {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl TryInto<std::fs::File> for &TestDataOut {
    type Error = std::io::Error;

    fn try_into(self) -> Result<std::fs::File, Self::Error> {
        std::fs::File::open(&self.0)
    }
}

impl TryInto<String> for &TestDataOut {
    type Error = std::io::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        let mut content = String::new();
        let mut file: std::fs::File = self.try_into()?;
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
