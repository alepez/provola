use chrono::Duration;

use crate::{report::Report, SingleReport, TestResult};

pub trait PendingReport {
    fn poll(&mut self) -> Option<Box<dyn Report>>;
}

pub struct ImmediatelyReadyPendingReport {
    report: Option<Box<dyn Report>>,
}

impl ImmediatelyReadyPendingReport {
    pub fn new(report: SingleReport) -> Self {
        Self {
            report: Some(Box::new(report)),
        }
    }
    pub fn from_result(result: TestResult, duration: Duration) -> Self {
        Self::new(SingleReport {
            result,
            duration: Some(duration),
        })
    }
}

impl PendingReport for ImmediatelyReadyPendingReport {
    fn poll(&mut self) -> Option<Box<dyn Report>> {
        self.report.take()
    }
}
