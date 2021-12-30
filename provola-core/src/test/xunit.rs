use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::iter::Enumerate;

use serde::{Deserialize, Serialize};

use crate::{CoreReport, CoreTestCase, CoreTestSuite};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TestSuite(pub String);

#[derive(Debug, Hash, Clone)]
pub struct TestCase(pub String);

#[derive(Debug, Clone)]
pub struct FullyQualifiedTestCase {
    pub test_suite: TestSuite,
    pub test_case: TestCase,
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

pub fn calculate_id(test_suite: &TestSuite, test_case: &TestCase) -> FullyQualifiedTestCaseId {
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FullyQualifiedTestCaseId(u64);

impl std::fmt::Display for FullyQualifiedTestCaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016X}", self.0)
    }
}

#[derive(Default, Debug)]
pub struct AvailableTests {
    list: Vec<FullyQualifiedTestCase>,
    map: BTreeMap<TestSuite, Vec<FullyQualifiedTestCase>>,
}

impl AvailableTests {
    pub fn push(&mut self, test_suite: impl Into<String>, test_case: impl Into<String>) {
        // TODO Optimization: prevent clone
        let fqtc = FullyQualifiedTestCase::from_test_suite_test_case(test_suite, test_case);
        self.list.push(fqtc.clone());
        self.map
            .entry(fqtc.test_suite.clone())
            .or_insert(Vec::default())
            .push(fqtc);
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn iter(&self) -> std::slice::Iter<FullyQualifiedTestCase> {
        self.list.iter()
    }

    pub fn enumerate(&self) -> Enumerate<std::slice::Iter<'_, FullyQualifiedTestCase>> {
        self.list.iter().enumerate()
    }

    pub fn get(&self, index: usize) -> Option<&FullyQualifiedTestCase> {
        self.list.get(index)
    }

    pub fn test_suites(
        &self,
    ) -> std::collections::btree_map::Iter<TestSuite, Vec<FullyQualifiedTestCase>> {
        self.map.iter()
    }
}

impl From<&AvailableTests> for CoreReport {
    fn from(available_tests: &AvailableTests) -> Self {
        let mut report = CoreReport::default();

        for (test_suite, test_cases) in available_tests.test_suites() {
            let name = test_suite.0.clone();

            let mut test_suite = CoreTestSuite {
                name,
                ..Default::default()
            };

            for fqtc in test_cases.iter() {
                let name = fqtc.test_case.0.clone();

                let test_case = CoreTestCase {
                    name,
                    ..Default::default()
                };

                test_suite.testcases.push(test_case);
            }

            report.testsuites.push(test_suite);
        }

        report
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
