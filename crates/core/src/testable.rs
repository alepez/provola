use crate::report::Report;
use std::time::Duration;

pub trait Testable: Send {
    fn run(&self) -> Report {
        Report::not_available()
    }

    fn is_ignored(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct DummyTestable {
    report: Report,
    delay: Duration,
}

impl DummyTestable {
    #[allow(dead_code)]
    pub fn new(report: Report) -> Self {
        Self::new_with_delay(report, Duration::from_secs(0))
    }
    #[allow(dead_code)]
    pub fn new_with_delay(report: Report, delay: Duration) -> Self {
        Self { report, delay }
    }
}

impl Testable for DummyTestable {
    fn run(&self) -> Report {
        std::thread::sleep(self.delay);
        self.report.clone()
    }
}
