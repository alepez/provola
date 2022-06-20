use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::report_future::ReportFuture;
use super::report::Report;

pub trait Testable: Send {
    fn run(&self) -> Report {
        Report::not_available()
    }

    fn is_ignored(&self) -> bool {
        false
    }
}

pub struct AsyncTestable(Arc<Mutex<Box<dyn Testable>>>);

impl AsyncTestable {
    fn new(testable: Box<dyn Testable>) -> Self {
        Self(Arc::new(Mutex::new(testable)))
    }

    fn launch(&self) -> ReportFuture {
        ReportFuture::new(self.0.clone())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn async_testable_can_be_launched() {
        let delay = Duration::from_millis(10);
        let testable = Box::new(DummyTestable::new_with_delay(Report::pass(), delay));
        let testable = AsyncTestable::new(testable);
        let f = testable.launch();
        let r = f.await;
        assert!(r.result.is_passed());
    }
}