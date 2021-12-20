use crate::{Error, TestResult};
use strum_macros::Display;

pub trait TestRunner {
    fn run(&self) -> Result<TestResult, Error> {
        Err(Error::TestRunnerFeatureNotAvailable(TestRunnerFeature::Run))
    }

    fn list(&self) -> Result<AvailableTests, Error> {
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

struct TestSuite(String);

struct TestCase(String);

struct FullyQualifiedTestCase {
    test_suite: TestSuite,
    test_case: TestCase,
}

pub struct AvailableTests(Vec<FullyQualifiedTestCase>);
