use provola_core::Named;
use provola_core::Testable;
use provola_core::{FailureDetails, Report, TestCase, TestSuite};

struct PassTestRunnerMock;

impl Testable for PassTestRunnerMock {
    fn run(&self) -> Report {
        Report::pass()
    }
}

struct FailTestRunnerMock;

impl Testable for FailTestRunnerMock {
    fn run(&self) -> Report {
        let details = FailureDetails {
            message: Some("oops!".into()),
            code_reference: None,
        };
        Report::fail_with_details(details)
    }
}

#[test]
fn test_custom_test_case() {
    let runner = Box::new(PassTestRunnerMock {});
    let test_case = TestCase::new("foo", runner);
    let report = test_case.run();
    assert!(report.result.is_passed());
    assert_eq!("foo", test_case.name());
}

#[test]
fn test_custom_test_case_failure() {
    let runner = Box::new(FailTestRunnerMock {});
    let test_case = TestCase::new("foo", runner);
    let report = test_case.run();
    assert!(report.result.is_failed());
}

#[test]
fn test_test_suite() {
    let test_suite = TestSuite::new("suite");
    assert_eq!("suite", test_suite.name.unwrap());
}
