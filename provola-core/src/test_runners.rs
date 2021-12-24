use crate::{Error, TestResult};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter::Enumerate;
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

#[derive(Debug, Hash)]
pub struct TestSuite(String);

#[derive(Debug, Hash)]
pub struct TestCase(String);

#[derive(Debug)]
pub struct FullyQualifiedTestCase {
    test_suite: TestSuite,
    test_case: TestCase,
    pub id: FullyQualifiedTestCaseId,
}

// id is already an hash of other members, so we can use just id to implement
// Hash trait
impl Hash for FullyQualifiedTestCase {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.id.0.hash(hasher);
    }
}

impl FullyQualifiedTestCase {
    pub fn from_test_suite_test_case(
        test_suite: impl Into<String>,
        test_case: impl Into<String>,
    ) -> Self {
        let test_suite = TestSuite(test_suite.into());
        let test_case = TestCase(test_case.into());
        Self::new(test_suite, test_case)
    }

    pub fn new(test_suite: TestSuite, test_case: TestCase) -> Self {
        let id = calculate_id(&test_suite, &test_case);
        Self {
            test_suite,
            test_case,
            id,
        }
    }
}

fn calculate_id(test_suite: &TestSuite, test_case: &TestCase) -> FullyQualifiedTestCaseId {
    let mut hasher = DefaultHasher::new();
    test_suite.hash(&mut hasher);
    test_case.hash(&mut hasher);
    let hash = hasher.finish();
    FullyQualifiedTestCaseId(hash)
}

impl std::fmt::Display for FullyQualifiedTestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.test_suite.0, self.test_case.0)
    }
}

#[derive(Debug)]
pub struct FullyQualifiedTestCaseId(u64);

impl std::fmt::Display for FullyQualifiedTestCaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

#[derive(Default, Debug)]
pub struct AvailableTests(Vec<FullyQualifiedTestCase>);

impl AvailableTests {
    pub fn push(&mut self, test_suite: impl Into<String>, test_case: impl Into<String>) {
        self.0
            .push(FullyQualifiedTestCase::from_test_suite_test_case(
                test_suite, test_case,
            ));
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

    pub fn enumerate(&self) -> Enumerate<std::slice::Iter<'_, FullyQualifiedTestCase>> {
        self.0.iter().enumerate()
    }
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct TestRunnerOpt {
    pub only: Only,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Only {
    SingleById(usize),
    All,
}

impl Default for Only {
    fn default() -> Self {
        Only::All
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn id_from_fqtc() {
        let fqtc = FullyQualifiedTestCase::from_test_suite_test_case("foo", "bar");

        let id = fqtc.id;

        assert_eq!(id.to_string(), "48730E17FEB9A107");
    }

    #[test]
    fn push_test_suite_test_case() {
        let mut available_tests = AvailableTests::default();
        assert!(available_tests.is_empty());
        available_tests.push("foo", "bar");
        assert_eq!(available_tests.len(), 1);
    }

    #[test]
    fn enumerate_available_tests() {
        let mut available_tests = AvailableTests::default();
        assert!(available_tests.is_empty());
        available_tests.push("foo", "bar");
        assert_eq!(available_tests.len(), 1);
    }
}
