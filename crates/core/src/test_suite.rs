use super::test_case::TestCase;
use crate::report::Report;
use crate::testable::Testable;

#[derive(Default)]
pub struct TestSuite {
    pub name: Option<String>,
    pub suites: Vec<TestSuite>,
    pub cases: Vec<TestCase>,
    pub ignored: bool,
}

impl Testable for TestSuite {
    fn start(&self) -> Report {
        if self.ignored {
            return Report::skipped();
        }

        let mut children = Vec::new();
        children.extend(self.cases.iter().map(|t| t.start()));
        children.extend(self.suites.iter().map(|t| t.start()));
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
    use super::*;
    use crate::report::TestResult;
    use crate::testable::DummyTestable;

    #[test]
    fn ignored_suite_is_skipped() {
        let s = TestSuite {
            name: None,
            suites: vec![],
            cases: vec![],
            ignored: true,
        };
        let r = s.start();
        assert!(matches!(r.result, TestResult::Skip));
    }

    #[test]
    fn suite_with_inner_suites_is_recursively_run() {
        let s = TestSuite {
            suites: vec![
                TestSuite {
                    ..Default::default()
                },
                TestSuite {
                    ..Default::default()
                },
            ],
            cases: vec![TestCase::new(
                "",
                Box::new(DummyTestable::new(Report::pass())),
            )],
            ..Default::default()
        };
        let r = s.start();
        assert!(matches!(r.result, TestResult::Pass));
    }
}
