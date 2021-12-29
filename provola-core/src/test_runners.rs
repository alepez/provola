use crate::test::xunit::AvailableTests;
use crate::{Error, TestResult};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

pub trait TestRunner {
    fn run(&self, _opt: &TestRunnerOpt) -> Result<TestResult, Error> {
        Err(Error::TestRunnerFeatureNotAvailable(TestRunnerFeature::Run))
    }

    fn list(&self, _opt: &TestRunnerOpt) -> Result<AvailableTests, Error> {
        Err(Error::TestRunnerFeatureNotAvailable(
            TestRunnerFeature::List,
        ))
    }
}

#[derive(Debug, Display)]
pub enum TestRunnerFeature {
    Run,
    List,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TestRunnerOpt {
    pub only: Only,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Only {
    SingleByIndex(usize),
    All,
}

impl Default for Only {
    fn default() -> Self {
        Only::All
    }
}
