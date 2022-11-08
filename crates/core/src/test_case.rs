use crate::named::Named;
use crate::report::PendingReport;
use crate::testable::Testable;

pub struct TestCase {
    name: String,
    runner: Box<dyn Testable>,
    ignored: bool,
}

impl Testable for TestCase {
    fn start(&self) -> Box<dyn PendingReport> {
        self.runner.start()
    }

    fn is_ignored(&self) -> bool {
        self.ignored
    }
}

impl Named for TestCase {
    fn name(&self) -> &str {
        &self.name
    }
}

impl TestCase {
    pub fn new(name: impl Into<String>, runner: Box<dyn Testable>) -> Self {
        Self {
            name: name.into(),
            runner,
            ignored: false,
        }
    }
}
