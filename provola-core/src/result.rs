#[derive(Debug)]
pub enum TestResult {
    Pass,
    Fail(Reason),
}

#[derive(Debug)]
pub enum Reason {
    Generic(String),
}

impl From<String> for Reason {
    fn from(text: String) -> Self {
        Reason::Generic(text)
    }
}
