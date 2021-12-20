use crate::{Error, TestResult};

pub trait TestRunner {
    fn run(&self) -> Result<TestResult, Error>;
}

