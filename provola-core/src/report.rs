use super::code::CodeReference;
use chrono::Duration;

pub struct FailureDetails {
    message: Option<String>,
    code_reference: Option<CodeReference>,
}

pub enum TestResult {
    Success,
    Failure(FailureDetails),
    Skipped,
}

pub struct Report {
    result: TestResult,
    duration: Option<Duration>,
    children: Vec<Report>,
}