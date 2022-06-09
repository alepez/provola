use super::test_case::TestCase;

struct TestSuite {
    name: Option<String>,
    suites: Vec<TestSuite>,
    test_cases: Vec<TestCase>,
}