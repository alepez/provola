use std::path::PathBuf;

#[derive(Debug)]
pub struct Source(PathBuf);

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

#[derive(Debug)]
pub struct TestDataOut(PathBuf);

impl TestDataOut {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

#[derive(Debug)]
pub enum Action {
    Build(Source),
    TestInputOutput(TestDataIn, TestDataOut),
}

#[derive(Debug)]
pub struct Actions(pub Vec<Action>);
