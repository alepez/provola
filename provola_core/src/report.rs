use std::ops::Add;
use super::code::CodeReference;
use super::error::Error;
use chrono::Duration;

#[derive(Default, Debug)]
pub struct FailureDetails {
    pub message: Option<String>,
    pub code_reference: Option<CodeReference>,
}

#[derive(Debug)]
pub enum TestResult {
    Passed,
    Failed(FailureDetails),
    Skipped,
    Aborted(Error),
    Mixed,
    Empty,
}

impl TestResult {
    pub fn is_passed(&self) -> bool {
        matches!(self, TestResult::Passed)
    }
    pub fn is_failed(&self) -> bool {
        matches!(self, TestResult::Failed(_))
    }
}

#[derive(Debug)]
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
        duration = duration.add(report.duration?);
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

    pub fn pass_with_duration(duration: Duration) -> Report {
        Report {
            result: TestResult::Passed,
            duration: Some(duration),
            children: Default::default(),
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

    #[test]
    fn test_sum_durations_of_nothings_gives_zero() {
        assert_eq!(Some(Duration::seconds(0)), sum_durations(&vec![]));
    }

    #[test]
    fn test_sum_durations_of_reports_with_duration() {
        let reports = vec![
            Report::pass_with_duration(Duration::seconds(1)),
            Report::pass_with_duration(Duration::seconds(2)),
            Report::pass_with_duration(Duration::milliseconds(3)),
        ];
        assert_eq!(Some(Duration::milliseconds(3003)), sum_durations(&reports));
    }

    #[test]
    fn test_sum_durations_of_reports_without_duration() {
        let reports = vec![
            Report::pass_with_duration(Duration::seconds(1)),
            Report::pass(),
        ];
        assert_eq!(None, sum_durations(&reports));
    }

    #[test]
    fn test_sum_durations_of_reports_with_children() {
        let reports = vec![
            Report::pass_with_duration(Duration::seconds(1)),
            Report::with_children(vec![
                Report::pass_with_duration(Duration::seconds(2)),
                Report::pass_with_duration(Duration::seconds(3)),
            ]),
        ];
        assert_eq!(Some(Duration::seconds(6)), sum_durations(&reports));
    }
}
