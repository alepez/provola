use provola_core::Reporter;
use provola_core::TestResult;

#[derive(Default)]
pub struct TerminalReporter;

impl TerminalReporter {
    pub fn new() -> Self {
        TerminalReporter {}
    }
}

impl Reporter for TerminalReporter {
    fn report(&self, result: TestResult) {
        if let TestResult::Fail(reason) = result {
            println!("FAIL\n");
            println!("{}", reason);
        } else {
            println!("PASS");
        }
    }
}
