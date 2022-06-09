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