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

impl Report {
    pub fn skipped() -> Report {
        Report {
            result: TestResult::Skipped,
            duration: None,
            children: Default::default(),
        }
    }

    pub fn with_children(children: Vec<Report>) -> Report {
        Report {
            result: TestResult::Skipped, // FIXME from children
            duration: None,// FIXME from children
            children,
        }
    }
}