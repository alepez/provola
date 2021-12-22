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
pub struct FullyQualifiedTestCase {
    test_suite: TestSuite,
    test_case: TestCase,
}

impl std::fmt::Display for FullyQualifiedTestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.test_suite.0, self.test_case.0)
    }
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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<FullyQualifiedTestCase> {
        self.0.iter()
    }
}
