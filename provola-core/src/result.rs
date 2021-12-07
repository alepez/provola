use std::fmt::Display;

use crate::Report;

#[derive(Debug)]
pub enum TestResult {
    Pass,
    Fail(Reason),
}

#[derive(Debug)]
pub enum Reason {
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

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Reason::Generic(description) => write!(f, "{}", description),
            Reason::NotExpected { actual, expected } => {
                write!(f, "Expected\n\n{}\n\nActual\n\n{}", expected, actual)
            }
            Reason::Report(report) => {
                // TODO Pretty print instead of debug
                write!(f, "{:#?}", &report)
            }
        }
    }
}

impl From<Report> for TestResult {
    fn from(x: Report) -> Self {
        let failures = x.failures.unwrap_or(0);
        if failures == 0 {
            TestResult::Pass
        } else {
            let reason = Reason::from_report(x);
            TestResult::Fail(reason)
        }
    }
}
