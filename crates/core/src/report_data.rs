use crate::code::CodeReference;
use crate::error::Error;

#[derive(Default, Debug, Clone)]
pub struct FailDetails {
    pub message: Option<String>,
    pub code_reference: Option<CodeReference>,
}

#[derive(Default, Debug, Clone)]
pub struct AbortDetails {
    pub error: Option<Error>,
}

#[derive(Debug, Clone)]
pub enum TestResult {
    Pass,
    Fail(Option<FailDetails>),
    Skip,
    Abort(Option<AbortDetails>),
    Mixed,
    Empty,
}
