use crate::TestResult;

pub trait Reporter {
    fn report(&self, result: TestResult);
}
