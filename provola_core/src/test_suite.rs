use crate::report::Report;
use crate::testable::Testable;
use super::test_case::TestCase;

#[derive(Default)]
pub struct TestSuite {
    pub name: Option<String>,
    pub suites: Vec<TestSuite>,
    pub cases: Vec<TestCase>,
    pub ignored: bool,
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
        self.ignored
    }
}

impl TestSuite {
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            suites: Default::default(),
            cases: Default::default(),
            ignored: false,
        }
    }
}