use super::code::CodeReference;
use chrono::Duration;

pub struct FailureDetails {
    pub message: Option<String>,
    pub code_reference: Option<CodeReference>,
}

pub enum TestResult {
    Pass,
    Fail(FailureDetails),
    Skipped,
}

impl TestResult {
    pub fn is_success(&self) -> bool {
        match self {
            TestResult::Pass => true,
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

    pub fn pass() -> Report {
        Report {
            result: TestResult::Pass,
            duration: None,
            children: Default::default(),
        }
    }

    pub fn fail(details: FailureDetails) -> Report {
        Report {
            result: TestResult::Fail(details),
            duration: None,
            children: Default::default(),
        }
    }
}