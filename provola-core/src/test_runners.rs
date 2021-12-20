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

#[derive(Debug)]
struct TestSuite(String);

#[derive(Debug)]
struct TestCase(String);

#[derive(Debug)]
struct FullyQualifiedTestCase {
    test_suite: TestSuite,
    test_case: TestCase,
}

#[derive(Default, Debug)]
pub struct AvailableTests(Vec<FullyQualifiedTestCase>);

impl AvailableTests {
    pub fn push(&mut self, test_suite: String, test_case: String) {
        let test_suite = TestSuite(test_suite);
        let test_case = TestCase(test_case);
        self.0.push(FullyQualifiedTestCase {
            test_suite,
            test_case,
        });
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
