use crate::code::CodeReference;
use crate::error::Error;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FailDetails {
    pub message: Option<String>,
    pub code_reference: Option<CodeReference>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AbortDetails {
    pub error: Option<Error>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestResult {
    Pass,
    Fail(Option<FailDetails>),
    Skip,
    Abort(Option<AbortDetails>),
    Mixed,
    Empty,
}
