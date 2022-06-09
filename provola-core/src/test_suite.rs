use crate::report::Report;
use crate::testable::Testable;
use super::test_case::TestCase;

struct TestSuite {
    name: Option<String>,
    suites: Vec<TestSuite>,
    cases: Vec<TestCase>,
    ignored: bool,
}

impl Testable for TestSuite {
    fn run(&self) -> Report {
        if self.ignored {
            return Report::skipped();
        }

        let mut children = Vec::new();
        children.extend(self.cases.iter().map(|t| t.run()));
        children.extend(self.suites.iter().map(|t| t.run()));
        Report::with_children(children)
    }

    fn is_ignored(&self) -> bool {
        todo!()
    }
}