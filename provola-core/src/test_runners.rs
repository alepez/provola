use crate::{Error, Executable, TestResult};

#[derive(Debug, strum_macros::EnumString, Clone, Copy)]
pub enum TestRunnerType {
    GoogleTest,
}

pub trait TestRunner {
    fn run(&self) -> Result<TestResult, Error>;
}

#[derive(Debug)]
pub struct TestRunnerInfo {
    pub exec: Executable,
    pub trt: TestRunnerType,
}

impl TestRunnerInfo {
    pub fn new(exec: Executable, trt: TestRunnerType) -> Self {
        Self { exec, trt }
    }
}
