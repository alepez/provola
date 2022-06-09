use std::ops::Add;
use super::code::CodeReference;
use super::error::Error;
use chrono::Duration;

#[derive(Default)]
pub struct FailureDetails {
    pub message: Option<String>,
    pub code_reference: Option<CodeReference>,
}

pub enum TestResult {
    Passed,
    Failed(FailureDetails),
    Skipped,
    Aborted(Error),
    Mixed,
}

impl TestResult {
    pub fn is_passed(&self) -> bool {
        matches!(self, TestResult::Passed)
    }
    pub fn is_failed(&self) -> bool {
        matches!(self, TestResult::Failed(_))
    }
}

pub struct Report {
    pub result: TestResult,
    pub duration: Option<Duration>,
    pub children: Vec<Report>,
}

fn fold_results(reports: &Vec<Report>) -> TestResult {
    let all_passed = reports.iter().all(|x| x.result.is_passed() && fold_results(&x.children).is_passed());

    if all_passed { return TestResult::Passed; }

    let all_failed = reports.iter().all(|x| x.result.is_failed() && fold_results(&x.children).is_failed());

    if all_failed { return TestResult::Failed(Default::default()); }

    TestResult::Mixed
}

fn sum_durations(reports: &Vec<Report>) -> Option<Duration> {
    let mut duration = Duration::seconds(0);

    for report in reports {
        duration.add(report.duration?);
        duration.add(sum_durations(&report.children)?);
    }

    Some(duration)
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
            duration: sum_durations(&children),
            children,
        }
    }

    pub fn pass() -> Report {
        Report {
            result: TestResult::Passed,
            duration: None,
            children: Default::default(),
        }
    }

    pub fn fail_with_details(details: FailureDetails) -> Report {
        Report {
            result: TestResult::Failed(details),
            duration: None,
            children: Default::default(),
        }
    }

    pub fn fail() -> Report {
        Report {
            result: TestResult::Failed(Default::default()),
            duration: None,
            children: Default::default(),
        }
    }

    pub fn not_available() -> Report {
        Report {
            result: TestResult::Aborted(Error::NotAvailable),
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

    #[test]
    fn test_report_with_only_passed() {
        let children = vec![
            Report::pass(),
            Report::pass(),
            Report::pass(),
        ];

        let report = Report::with_children(children);
        assert!(matches!(report.result, TestResult::Passed));
    }
}
