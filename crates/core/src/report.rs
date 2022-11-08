use super::code::CodeReference;
use super::error::Error;
use chrono::Duration;
use std::ops::Add;

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
    Fail(FailDetails),
    Skip,
    Abort(AbortDetails),
    Mixed,
    Empty,
}

impl TestResult {
    pub fn is_passed(&self) -> bool {
        matches!(self, TestResult::Pass)
    }
    pub fn is_failed(&self) -> bool {
        matches!(self, TestResult::Fail(_))
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub result: TestResult,
    pub duration: Option<Duration>,
    pub children: Vec<Report>,
}

fn fold_results(reports: &[Report]) -> TestResult {
    let all_passed = reports
        .iter()
        .all(|x| x.result.is_passed() && fold_results(&x.children).is_passed());

    if all_passed {
        return TestResult::Pass;
    }

    let all_failed = reports
        .iter()
        .all(|x| x.result.is_failed() && fold_results(&x.children).is_failed());

    if all_failed {
        return TestResult::Fail(Default::default());
    }

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
            result: TestResult::Skip,
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
            result: TestResult::Pass,
            duration: Some(duration),
            children: Default::default(),
        }
    }

    pub fn pass() -> Report {
        Report {
            result: TestResult::Pass,
            duration: None,
            children: Default::default(),
        }
    }

    pub fn fail_with_details(details: FailDetails) -> Report {
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
            result: TestResult::Abort(AbortDetails {
                error: Some(Error::NotAvailable),
            }),
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
        let children = vec![Report::skipped(), Report::pass(), Report::fail()];

        let report = Report::with_children(children);
        assert!(matches!(report.result, TestResult::Mixed));
    }

    #[test]
    fn test_report_with_only_passed() {
        let children = vec![Report::pass(), Report::pass(), Report::pass()];

        let report = Report::with_children(children);
        assert!(matches!(report.result, TestResult::Pass));
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
