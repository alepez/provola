use crate::{Error, TestResult};
use strum_macros::Display;

pub trait TestRunner {
    fn run(&self) -> Result<TestResult, Error> {
        Err(Error::TestRunnerFeatureNotAvailable(TestRunnerFeature::Run))
    }
}

#[derive(Debug, Display)]
pub enum TestRunnerFeature {
    Run,
}
