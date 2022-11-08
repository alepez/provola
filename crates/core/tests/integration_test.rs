use chrono::Duration;

use provola_core::*;

struct ImmediatelyReadyPendingReport {
    report: Option<Box<dyn Report>>,
}

impl ImmediatelyReadyPendingReport {
    fn new(report: SingleReport) -> Self {
        Self {
            report: Some(Box::new(report)),
        }
    }
}

impl PendingReport for ImmediatelyReadyPendingReport {
    fn poll(&mut self) -> Option<Box<dyn Report>> {
        self.report.take()
    }
}

struct PassTestCaseMock;

impl Ignorable for PassTestCaseMock {}

impl Testable for PassTestCaseMock {
    fn start(&self) -> Box<dyn PendingReport> {
        Box::new(ImmediatelyReadyPendingReport::new(SingleReport {
            result: TestResult::Pass,
            duration: Some(Duration::seconds(1)),
        }))
    }
}

impl Named for PassTestCaseMock {
    fn name(&self) -> &str {
        "foo"
    }
}

impl TestCase for PassTestCaseMock {}

struct FailTestCaseMock;

impl Ignorable for FailTestCaseMock {}

impl Testable for FailTestCaseMock {
    fn start(&self) -> Box<dyn PendingReport> {
        Box::new(ImmediatelyReadyPendingReport::new(SingleReport {
            result: TestResult::Fail(None),
            duration: Some(Duration::seconds(1)),
        }))
    }
}

impl TestCase for FailTestCaseMock {}

impl Named for FailTestCaseMock {
    fn name(&self) -> &str {
        "foo"
    }
}

#[test]
fn test_custom_test_case() {
    let test_case = PassTestCaseMock;
    let report = test_case.start().poll().unwrap();
    assert!(report.result().is_passed());
    assert_eq!("foo", test_case.name());
}

#[test]
fn test_custom_test_case_failure() {
    let test_case = FailTestCaseMock;
    let report = test_case.start().poll().unwrap();
    assert!(report.result().is_failed());
}

#[test]
fn test_test_suite() {
    // let test_suite = TestSuite::new("suite");
    // assert_eq!("suite", test_suite.name.unwrap());
}
