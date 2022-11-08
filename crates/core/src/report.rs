use super::code::CodeReference;
use super::error::Error;
use chrono::Duration;
use std::ops::Add;

pub trait Report {
    fn result(&self) -> &TestResult;
    fn duration(&self) -> Option<Duration>;
}

pub trait PendingReport {
    fn poll(&self) -> Option<TreeReport>;
}

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
    Fail(Option<FailDetails>),
    Skip,
    Abort(Option<AbortDetails>),
    Mixed,
    Empty,
}

#[derive(Debug, Clone)]
pub struct SingleReport {
    pub result: TestResult,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct MultiReport {
    pub children: Vec<SingleReport>,
}

#[derive(Debug, Clone)]
pub enum TreeReport {
    Single(SingleReport),
    Multi(MultiReport),
}

impl TestResult {
    pub fn is_passed(&self) -> bool {
        matches!(self, TestResult::Pass)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, TestResult::Fail(_))
    }
}

impl Report for SingleReport {
    fn result(&self) -> &TestResult {
        &self.result
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
    fn result(&self) -> &TestResult {
        let _result = self.fold_results();
        todo!()
    }

    fn duration(&self) -> Option<Duration> {
        self.sum_durations()
    }
}

impl MultiReport {
    pub fn new(children: Vec<SingleReport>) -> Self {
        Self { children }
    }

    fn fold_results(&self) -> TestResult {
        let all_passed = self.children.iter().all(|x| x.result.is_passed());
        let all_failed = self.children.iter().all(|x| x.result.is_failed());

        match (all_passed, all_failed) {
            (true, false) => TestResult::Pass,
            (false, true) => TestResult::Fail(None),
            _ => TestResult::Mixed,
        }
    }

    fn sum_durations(&self) -> Option<Duration> {
        let mut duration = Duration::seconds(0);

        for report in &self.children {
            duration = duration.add(report.duration?);
        }

        Some(duration)
    }
}

impl Report for TreeReport {
    fn result(&self) -> &TestResult {
        match self {
            TreeReport::Single(x) => x.result(),
            TreeReport::Multi(x) => x.result(),
        }
    }

    fn duration(&self) -> Option<Duration> {
        match self {
            TreeReport::Single(x) => x.duration(),
            TreeReport::Multi(x) => x.duration(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_report_with_children() {
    //     let children = vec![
    //         SingleReport::skipped(),
    //         SingleReport::pass(),
    //         SingleReport::fail(),
    //     ];

    //     let report = MultiReport::new(children);
    //     assert!(matches!(report.result, TestResult::Mixed));
    // }

    // #[test]
    // fn test_report_with_only_passed() {
    //     let children = vec![
    //         SingleReport::pass(),
    //         SingleReport::pass(),
    //         SingleReport::pass(),
    //     ];

    //     let report = MultiReport::new(children);
    //     assert!(matches!(report.result, TestResult::Pass));
    // }

    // #[test]
    // fn test_sum_durations_of_nothings_gives_zero() {
    //     assert_eq!(Some(Duration::seconds(0)), sum_durations(&vec![]));
    // }

    // #[test]
    // fn test_sum_durations_of_reports_with_duration() {
    //     let reports = vec![
    //         SingleReport::pass_with_duration(Duration::seconds(1)),
    //         SingleReport::pass_with_duration(Duration::seconds(2)),
    //         SingleReport::pass_with_duration(Duration::milliseconds(3)),
    //     ];
    //     let multi = MultiReport::new(reports);
    //     assert_eq!(Some(Duration::milliseconds(3003)), sum_durations(&reports));
    // }

    // #[test]
    // fn test_sum_durations_of_reports_without_duration() {
    //     let reports = vec![
    //         SingleReport::pass_with_duration(Duration::seconds(1)),
    //         SingleReport::pass(),
    //     ];
    //     let multi = MultiReport::new(reports);
    //     assert_eq!(None, multi.(&reports));
    // }

    // #[test]
    // fn test_sum_durations_of_reports_with_children() {
    //     let reports = vec![
    //         Report::pass_with_duration(Duration::seconds(1)),
    //         Report::new(vec![
    //             Report::pass_with_duration(Duration::seconds(2)),
    //             Report::pass_with_duration(Duration::seconds(3)),
    //         ]),
    //     ];
    //     assert_eq!(Some(Duration::seconds(6)), sum_durations(&reports));
    // }
}
