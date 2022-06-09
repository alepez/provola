use provola_core::{Report, TestCase, TestSuite, FailureDetails};
use provola_core::Testable;
use provola_core::Named;

struct PassTestRunnerMock;

impl Testable for PassTestRunnerMock {
    fn run(&self) -> Report {
        Report::pass()
    }

    fn is_ignored(&self) -> bool {
        false
    }
}

struct FailTestRunnerMock;

impl Testable for FailTestRunnerMock {
    fn run(&self) -> Report {
        let details = FailureDetails { message: Some("oops!".into()), code_reference: None };
        Report::fail(details)
    }

    fn is_ignored(&self) -> bool {
        false
    }
}

#[test]
fn test_custom_test_case() {
    let runner = Box::new(PassTestRunnerMock {});
    let test_case = TestCase::new("foo".into(), runner);
    let report = test_case.run();
    assert!(report.result.is_success());
    assert_eq!("foo", test_case.name());
}

#[test]
fn test_custom_test_case_failure() {
    let runner = Box::new(FailTestRunnerMock {});
    let test_case = TestCase::new("foo".into(), runner);
    let report = test_case.run();
    assert!(report.result.is_fail());
}

#[test]
fn test_test_suite() {
    let test_suite = TestSuite::new("suite".into());
    assert_eq!("suite", test_suite.name.unwrap());
}