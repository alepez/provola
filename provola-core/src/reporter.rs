use crate::TestResult;

pub trait Reporter {
    fn report(&self, result: TestResult);
}

#[derive(Default)]
pub struct SimpleReporter;

impl SimpleReporter {
    pub fn new() -> Self {
        SimpleReporter {}
    }
}

impl Reporter for SimpleReporter {
    fn report(&self, result: TestResult) {
        if let TestResult::Fail(reason) = result {
            println!("FAIL\n");
            println!("{}", reason);
        } else {
            println!("PASS");
        }
    }
}
