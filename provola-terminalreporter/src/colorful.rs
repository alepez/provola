use provola_core::Reason;
use provola_core::Reporter;
use provola_core::ReporterError;
use provola_core::TestResult;
use std::io::Write;

#[derive(Default)]
pub struct ThisReporter;

trait ThisDisplay: Sized {
    fn tr_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn to_tr_wrapper(&self) -> ThisWrapper<Self> {
        ThisWrapper(self)
    }
}

struct ThisWrapper<'a, T: ThisDisplay>(&'a T);

impl<T> std::fmt::Display for ThisWrapper<'_, T>
where
    T: ThisDisplay,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.tr_fmt(f)
    }
}

impl ThisReporter {
    pub fn new() -> Self {
        ThisReporter {}
    }
}

impl Reporter for ThisReporter {
    fn report(&self, result: TestResult) -> Result<(), ReporterError> {
        let mut writer = std::io::stdout();
        write!(writer, "{}", result.to_tr_wrapper()).map_err(ReporterError::IoError)
    }
}

impl ThisDisplay for TestResult {
    fn tr_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let TestResult::Fail(reason) = &self {
            writeln!(f, "FAIL\n")?;
            writeln!(f, "{}", reason.to_tr_wrapper())?;
            Ok(())
        } else {
            writeln!(f, "PASS")
        }
    }
}

impl ThisDisplay for Reason {
    fn tr_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Reason::Unknown => std::write!(f, ""),
            Reason::Generic(description) => std::write!(f, "{}", description),
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn report_pass() {
        let mut s = String::new();
        let res = TestResult::Pass;
        let res = res.to_tr_wrapper();
        write!(s, "{}", res).unwrap();
        insta::assert_debug_snapshot!(s);
    }

    #[test]
    fn report_fail_reason() {
        let mut s = String::new();
        let reason = Reason::not_expected("foo", "bar");
        let res = TestResult::Fail(reason);
        let res = res.to_tr_wrapper();
        write!(s, "{}", res).unwrap();
        insta::assert_debug_snapshot!(s);
    }
}
