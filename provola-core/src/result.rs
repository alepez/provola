#[derive(Debug)]
pub enum TestResult {
    Pass,
    Fail(Reason),
}

#[derive(Debug)]
pub struct Reason {
    text: String,
}

impl From<String> for Reason {
    fn from(text: String) -> Self {
        Reason { text }
    }
}
