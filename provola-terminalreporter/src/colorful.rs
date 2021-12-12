use colored::*;
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

impl Reporter for ThisReporter {
    fn report(&self, result: TestResult) -> Result<(), ReporterError> {
        let mut writer = std::io::stdout();
        write!(writer, "{}", result.to_tr_wrapper()).map_err(ReporterError::IoError)
    }
}

impl ThisDisplay for TestResult {
    fn tr_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TestResult::Fail(reason) => {
                writeln!(f, "{}", reason.to_tr_wrapper())?;
                writeln!(f, "{}", "FAIL".red().bold())?;
            }
            TestResult::Pass(reason) => {
                writeln!(f, "{}", reason.to_tr_wrapper())?;
                writeln!(f, "{}", "PASS".green().bold())?;
            }
        }
        Ok(())
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
                    writeln!(f, "{}", name.bold())?;
                }

                for testsuite in &report.testsuites {
                    writeln!(f, "  {}", testsuite.name.bold())?;

                    for testcase in &testsuite.testcases {
                        let ok = testcase.failures.is_empty();
                        let symbol = if ok { "✔".green() } else { "✖".red() };
                        writeln!(f, "    {} {}", symbol, testcase.name)?;
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
