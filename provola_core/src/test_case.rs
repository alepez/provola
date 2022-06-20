use crate::named::Named;
use crate::report::Report;
use crate::testable::Testable;

pub struct TestCase {
    name: String,
    runner: Box<dyn Testable>,
    ignored: bool,
}

impl Testable for TestCase {
    fn run(&self) -> Report {
        self.runner.run()
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