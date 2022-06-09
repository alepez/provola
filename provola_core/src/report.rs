use super::code::CodeReference;
use super::error::Error;
use chrono::Duration;

#[derive(Default)]
pub struct FailureDetails {
    pub message: Option<String>,
    pub code_reference: Option<CodeReference>,
}

pub enum TestResult {
    Pass,
    Fail(FailureDetails),
    Skipped,
    Error(Error),
    Mixed,
}

impl TestResult {
    pub fn is_success(&self) -> bool {
        match self {
            TestResult::Pass => true,
            _ => false,
        }
    }
    pub fn is_fail(&self) -> bool {
        match self {
            TestResult::Fail(_) => true,
            _ => false,
        }
    }
}

pub struct Report {
    pub result: TestResult,
    pub duration: Option<Duration>,
    pub children: Vec<Report>,
}

fn fold_results(reports: &Vec<Report>) -> TestResult {
    let all_passed = reports.iter().all(|x| x.result.is_success() && fold_results(&x.children).is_success());

    if all_passed { return TestResult::Pass; }

    let all_failed = reports.iter().all(|x| x.result.is_fail() && fold_results(&x.children).is_fail());

    if all_failed { return TestResult::Fail(Default::default()); }

    TestResult::Mixed
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
            result: fold_results(&children),
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

    pub fn fail_with_details(details: FailureDetails) -> Report {
        Report {
            result: TestResult::Fail(details),
            duration: None,
            children: Default::default(),
        }
    }

    pub fn fail() -> Report {
        Report {
            result: TestResult::Fail(Default::default()),
            duration: None,
            children: Default::default(),
        }
    }

    pub fn not_available() -> Report {
        Report {
            result: TestResult::Error(Error::NotAvailable),
            duration: None,
            children: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_with_children() {
        let children = vec![
            Report::skipped(),
            Report::pass(),
            Report::fail(),
        ];

        let report = Report::with_children(children);
        assert!(matches!(report.result, TestResult::Mixed));
    }
}
