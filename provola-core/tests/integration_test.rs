use provola_core::{Report, TestCase, TestSuite};
use provola_core::Testable;
use provola_core::Named;

struct TestRunnerMock;

impl Testable for TestRunnerMock {
    fn run(&self) -> provola_core::Report {
        Report::success()
    }

    fn is_ignored(&self) -> bool {
        false
    }
}

#[test]
fn test_custom_test_case() {
    let runner = Box::new(TestRunnerMock {});
    let test_case = TestCase::new("foo".into(), runner);
    let report = test_case.run();
    assert!(report.result.is_success());
    assert_eq!("foo", test_case.name());
}

#[test]
fn test_test_suite() {
    let test_suite = TestSuite::new("suite".into());
    assert_eq!("suite", test_suite.name.unwrap());
}