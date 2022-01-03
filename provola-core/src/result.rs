use std::fmt::Display;

use crate::CoreReport;

#[derive(Debug, Clone)]
pub enum TestResult {
    Pass(Reason),
    Fail(Reason),
}

#[derive(Debug, Clone)]
pub enum Reason {
    Unknown,
    Generic(String),
    NotExpected { actual: String, expected: String },
    Report(CoreReport),
}

impl From<String> for Reason {
    fn from(text: String) -> Self {
        Reason::Generic(text)
    }
}

impl Reason {
    pub fn not_expected(actual: impl Display, expected: impl Display) -> Self {
        let actual = actual.to_string();
        let expected = expected.to_string();
        Reason::NotExpected { actual, expected }
    }

    fn from_report(report: CoreReport) -> Self {
        Reason::Report(report)
    }
}

impl From<CoreReport> for TestResult {
    fn from(x: CoreReport) -> Self {
        let failures = x.failures.unwrap_or(0);
        let reason = Reason::from_report(x);
        if failures == 0 {
            TestResult::Pass(reason)
        } else {
            TestResult::Fail(reason)
        }
    }
}
