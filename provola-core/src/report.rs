use super::code::CodeReference;

pub struct FailureDetails {
    message: String,
    code_reference: Option<CodeReference>,
}

pub enum TestResult {
    Success,
    Failure(FailureDetails),
}

pub struct Report {
    result: TestResult,
}