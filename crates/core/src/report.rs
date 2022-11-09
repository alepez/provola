use crate::{
    error::Error,
    report_data::{AbortDetails, FailDetails, TestResult},
};
use chrono::Duration;
use std::ops::Add;

pub trait Report: core::fmt::Debug {
    fn result(&self) -> TestResult;
    fn duration(&self) -> Option<Duration>;
}

#[derive(Debug, Clone)]
pub struct SingleReport {
    pub result: TestResult,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct MultiReport {
    pub children: Vec<NodeReport>,
}

#[derive(Debug, Clone)]
pub enum NodeReport {
    Single(SingleReport),
    Multi(MultiReport),
}

impl Report for SingleReport {
    fn result(&self) -> TestResult {
        self.result.clone()
    }

    fn duration(&self) -> Option<Duration> {
        self.duration
    }
}

impl SingleReport {
    pub fn skipped() -> Self {
        Self {
            result: TestResult::Skip,
            duration: None,
        }
    }

    pub fn pass_with_duration(duration: Duration) -> Self {
        Self {
            result: TestResult::Pass,
            duration: Some(duration),
        }
    }

    pub fn pass() -> Self {
        Self {
            result: TestResult::Pass,
            duration: None,
        }
    }

    pub fn fail_with_details(details: FailDetails) -> Self {
        Self {
            result: TestResult::Fail(Some(details)),
            duration: None,
        }
    }

    pub fn fail() -> Self {
        Self {
            result: TestResult::Fail(None),
            duration: None,
        }
    }

    pub fn not_available() -> Self {
        let error = Error::NotAvailable;
        let details = AbortDetails { error: Some(error) };
        Self {
            result: TestResult::Abort(Some(details)),
            duration: None,
        }
    }
}

impl Report for MultiReport {
    fn result(&self) -> TestResult {
        fold_results(self.children.iter())
    }

    fn duration(&self) -> Option<Duration> {
        sum_durations(self.children.iter())
    }
}

impl MultiReport {
    pub fn new(children: impl IntoIterator<Item = impl Into<NodeReport>>) -> Self {
        let children = children.into_iter().map(|x| x.into()).collect();
        Self { children }
    }
}

impl Report for &NodeReport {
    fn result(&self) -> TestResult {
        match self {
            NodeReport::Single(x) => x.result(),
            NodeReport::Multi(x) => x.result(),
        }
    }

    fn duration(&self) -> Option<Duration> {
        match self {
            NodeReport::Single(x) => x.duration(),
            NodeReport::Multi(x) => x.duration(),
        }
    }
}

impl From<SingleReport> for NodeReport {
    fn from(x: SingleReport) -> Self {
        NodeReport::Single(x)
    }
}

impl From<MultiReport> for NodeReport {
    fn from(x: MultiReport) -> Self {
        NodeReport::Multi(x)
    }
}

fn fold_results<T>(mut reports: T) -> TestResult
where
    T: Iterator + Clone,
    T::Item: Report,
{
    let all_passed = reports.clone().all(|x| x.result().is_passed());
    let all_failed = reports.all(|x| x.result().is_failed());

    match (all_passed, all_failed) {
        (true, false) => TestResult::Pass,
        (false, true) => TestResult::Fail(None),
        _ => TestResult::Mixed,
    }
}

fn sum_durations<T>(reports: T) -> Option<Duration>
where
    T: Iterator,
    T::Item: Report,
{
    let mut duration = Duration::seconds(0);

    for report in reports {
        duration = duration.add(report.duration()?);
    }

    Some(duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_with_children() {
        let children = vec![
            SingleReport::skipped(),
            SingleReport::pass(),
            SingleReport::fail(),
        ];

        let report = MultiReport::new(children);
        assert!(matches!(report.result(), TestResult::Mixed));
    }

    #[test]
    fn test_report_with_only_passed() {
        let children = vec![
            SingleReport::pass(),
            SingleReport::pass(),
            SingleReport::pass(),
        ];

        let report = MultiReport::new(children);
        assert!(matches!(report.result(), TestResult::Pass));
    }

    #[test]
    fn test_sum_durations_of_nothings_gives_zero() {
        let children: Vec<SingleReport> = vec![];
        assert_eq!(
            Some(Duration::seconds(0)),
            sum_durations(children.into_iter())
        );
    }

    #[test]
    fn test_sum_durations_of_reports_with_duration() {
        let reports = vec![
            SingleReport::pass_with_duration(Duration::seconds(1)),
            SingleReport::pass_with_duration(Duration::seconds(2)),
            SingleReport::pass_with_duration(Duration::milliseconds(3)),
        ];
        let multi = MultiReport::new(reports);
        assert_eq!(Some(Duration::milliseconds(3003)), multi.duration());
    }

    #[test]
    fn test_sum_durations_of_reports_without_duration() {
        let reports = vec![
            SingleReport::pass_with_duration(Duration::seconds(1)),
            SingleReport::pass(),
        ];
        let multi = MultiReport::new(reports);
        assert_eq!(None, multi.duration());
    }

    #[test]
    fn test_sum_durations_of_reports_with_children() {
        let reports: Vec<NodeReport> = vec![
            SingleReport::pass_with_duration(Duration::seconds(1)).into(),
            MultiReport::new(vec![
                SingleReport::pass_with_duration(Duration::seconds(2)),
                SingleReport::pass_with_duration(Duration::seconds(3)),
            ])
            .into(),
        ];
        let report = MultiReport::new(reports);
        assert_eq!(Some(Duration::seconds(6)), report.duration());
    }
}
