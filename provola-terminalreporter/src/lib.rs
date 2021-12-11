use provola_core::Reason;
use provola_core::Reporter;
use provola_core::TestResult;

#[derive(Default)]
pub struct TerminalReporter;

trait TerminalReporterDisplay {
    fn print(&self);
}

impl TerminalReporter {
    pub fn new() -> Self {
        TerminalReporter {}
    }
}

impl Reporter for TerminalReporter {
    fn report(&self, result: TestResult) {
        if let TestResult::Fail(reason) = result {
            println!("FAIL\n");
            reason.print();
        } else {
            println!("PASS");
        }
    }
}

impl TerminalReporterDisplay for Reason {
    fn print(&self) {
        match &self {
            Reason::Generic(description) => print!("{}", description),
            Reason::NotExpected { actual, expected } => {
                print!("Expected\n\n{}\n\nActual\n\n{}", expected, actual)
            }
            Reason::Report(report) => {
                if let Some(name) = &report.name {
                    print!("{} | ", name);
                }

                if let Some(tests) = report.tests {
                    print!("tests: {} | ", tests);
                }

                if let Some(errors) = report.errors {
                    print!("errors: {} | ", errors);
                }

                if let Some(failures) = report.failures {
                    print!("failures: {} | ", failures);
                }

                if let Some(disabled) = report.disabled {
                    print!("- disabled: {} | ", disabled);
                }

                println!();

                for testsuite in &report.testsuites {
                    print!("    {} | ", testsuite.name);

                    print!("tests: {} | ", testsuite.tests);

                    if let Some(errors) = testsuite.errors {
                        print!("errors: {} | ", errors);
                    }

                    if let Some(failures) = testsuite.failures {
                        print!("failures: {} | ", failures);
                    }

                    if let Some(disabled) = testsuite.disabled {
                        print!("disabled: {} | ", disabled);
                    }

                    println!();

                    for testcase in &testsuite.testcases {
                        let ok = testcase.failures.is_empty();
                        let result = if ok { "PASS" } else { "FAIL" };
                        println!("        {} {}", testcase.name, result);
                    }
                }
            }
        }
    }
}
