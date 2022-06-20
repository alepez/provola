use super::report::Report;

pub trait Testable {
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
}

impl DummyTestable {
    #[allow(dead_code)]
    pub fn new(report: Report) -> Self {
        Self { report }
    }
}

impl Testable for DummyTestable {
    fn run(&self) -> Report {
        self.report.clone()
    }
}
