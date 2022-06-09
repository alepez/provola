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

impl TestResult {
    pub fn is_success(&self) -> bool {
        match self {
            TestResult::Success => true,
            _ => false,
        }
    }
}

pub struct Report {
    pub result: TestResult,
    pub duration: Option<Duration>,
    pub children: Vec<Report>,
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

    pub fn success() -> Report {
        Report {
            result: TestResult::Success,
            duration: None,
            children: Default::default(),
        }
    }
}