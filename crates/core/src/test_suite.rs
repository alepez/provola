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
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            suites: Default::default(),
            cases: Default::default(),
            ignored: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::report::TestResult;
    use crate::testable::DummyTestable;
    use super::*;

    #[test]
    fn ignored_suite_is_skipped() {
        let s = TestSuite { name: None, suites: vec![], cases: vec![], ignored: true };
        let r = s.run();
        assert!(matches!(r.result, TestResult::Skipped));
    }

    #[test]
    fn suite_with_inner_suites_is_recursively_run() {
        let s = TestSuite {
            suites: vec![
                TestSuite { ..Default::default() },
                TestSuite { ..Default::default() },
            ],
            cases: vec![
                TestCase::new("", Box::new(DummyTestable::new(Report::pass()))),
            ],
            ..Default::default()
        };
        let r = s.run();
        assert!(matches!(r.result, TestResult::Passed));
    }
}