use super::test_case::TestCase;
use crate::report::PendingReport;
use crate::testable::Testable;

#[derive(Default)]
pub struct TestSuite {
    pub name: Option<String>,
    pub suites: Vec<TestSuite>,
    pub cases: Vec<TestCase>,
    pub ignored: bool,
}

impl Testable for TestSuite {
    fn start(&self) -> Box<dyn PendingReport> {
        if self.ignored {
            todo!()
        }

        todo!()
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::report::TestResult;

//     #[test]
//     fn ignored_suite_is_skipped() {
//         let s = TestSuite {
//             name: None,
//             suites: vec![],
//             cases: vec![],
//             ignored: true,
//         };
//         let r = s.start();
//         assert!(matches!(r.result, TestResult::Skip));
//     }

//     #[test]
//     fn suite_with_inner_suites_is_recursively_run() {
//         let s = TestSuite {
//             suites: vec![
//                 TestSuite {
//                     ..Default::default()
//                 },
//                 TestSuite {
//                     ..Default::default()
//                 },
//             ],
//             cases: vec![TestCase::new(
//                 "",
//                 Box::new(DummyTestable::new(Report::pass())),
//             )],
//             ..Default::default()
//         };
//         let r = s.start();
//         assert!(matches!(r.result, TestResult::Pass));
//     }
// }
