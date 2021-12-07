use crate::{Error, Executable, TestResult};

#[derive(Debug, strum_macros::EnumString, Clone, Copy)]
pub enum TestRunnerType {
    GoogleTest,
}

pub struct TestRunner {
    exec: Executable,
    trt: TestRunnerType,
}

impl TestRunner {
    pub fn new(exec: Executable, trt: TestRunnerType) -> Self {
        Self { exec, trt }
    }

    pub fn run(&self) -> Result<TestResult, Error> {
        match self.trt {
            TestRunnerType::GoogleTest => {
                todo!()
            }
        }
    }
}
