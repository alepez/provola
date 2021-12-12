use std::fmt::Display;

use crate::Report;

#[derive(Debug)]
pub enum TestResult {
    Pass(Reason),
    Fail(Reason),
}

#[derive(Debug)]
pub enum Reason {
    Unknown,
    Generic(String),
    NotExpected { actual: String, expected: String },
    Report(Report),
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

    fn from_report(report: Report) -> Self {
        Reason::Report(report)
    }
}

impl From<Report> for TestResult {
    fn from(x: Report) -> Self {
        let failures = x.failures.unwrap_or(0);
        let reason = Reason::from_report(x);
        if failures == 0 {
            TestResult::Pass(reason)
        } else {
            TestResult::Fail(reason)
        }
    }
}
