#[derive(Debug)]
pub enum TestResult {
    Pass,
    Fail,
}

impl From<bool> for TestResult {
    fn from(x: bool) -> Self {
        if x {
            TestResult::Pass
        } else {
            TestResult::Fail
        }
    }
}
