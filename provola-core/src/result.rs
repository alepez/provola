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
                if let Some(name) = &report.name {
                    write!(f, "{} | ", name)?;
                }

                if let Some(tests) = report.tests {
                    write!(f, "tests: {} | ", tests)?;
                }

                if let Some(errors) = report.errors {
                    write!(f, "errors: {} | ", errors)?;
                }

                if let Some(failures) = report.failures {
                    write!(f, "failures: {} | ", failures)?;
                }

                if let Some(disabled) = report.disabled {
                    write!(f, "- disabled: {} | ", disabled)?;
                }

                writeln!(f)?;

                for testsuite in &report.testsuites {
                    write!(f, "    {} | ", testsuite.name)?;

                    write!(f, "tests: {} | ", testsuite.tests)?;

                    if let Some(errors) = testsuite.errors {
                        write!(f, "errors: {} | ", errors)?;
                    }

                    if let Some(failures) = testsuite.failures {
                        write!(f, "failures: {} | ", failures)?;
                    }

                    if let Some(disabled) = testsuite.disabled {
                        write!(f, "disabled: {} | ", disabled)?;
                    }

                    writeln!(f)?;

                    for testcase in &testsuite.testcases {
                        let ok = testcase.failures.is_empty();
                        let result = if ok { "PASS" } else { "FAIL" };
                        writeln!(f, "        {} {}", testcase.name, result)?;
                    }
                }

                Ok(())
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
