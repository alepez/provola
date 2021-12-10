use crate::{Error, TestResult, Executable};

pub trait TestRunner {
    fn from_executable(executable: Executable) -> Box<dyn TestRunner>
    where
        Self: Sized;
    fn run(&self) -> Result<TestResult, Error>;
}

