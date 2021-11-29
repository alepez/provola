use std::fmt::Display;

#[derive(Debug)]
pub enum TestResult {
    Pass,
    Fail(Reason),
}

#[derive(Debug)]
pub enum Reason {
    Generic(String),
    NotExpected { actual: String, expected: String },
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
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Reason::Generic(description) => write!(f, "{}", description),
            Reason::NotExpected { actual, expected } => {
                write!(f, "Expected\n\n{}\n\nActual\n\n{}", expected, actual)
            }
        }
    }
}
