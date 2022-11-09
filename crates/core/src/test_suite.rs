use crate::{Ignorable, Named, Runner, TestCase, Testable};

#[derive(Debug)]
pub struct SingleTestCase {
    name: String,
    runner: Box<dyn Runner>,
}

#[derive(Debug)]
pub struct MultiTestCase {
    name: String,
    #[allow(dead_code)]
    children: Vec<NodeTestCase>,
}

#[derive(Debug)]
pub enum NodeTestCase {
    Single(SingleTestCase),
    Multi(MultiTestCase),
}

impl Ignorable for SingleTestCase {}

impl Named for SingleTestCase {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Testable for SingleTestCase {
    fn start(&self) -> Box<dyn crate::PendingReport> {
        self.runner.start()
    }
}

impl TestCase for SingleTestCase {}

impl SingleTestCase {
    pub fn new(name: impl Into<String>, runner: Box<dyn Runner>) -> Self {
        let name: String = name.into();
        Self { name, runner }
    }
}

impl Ignorable for MultiTestCase {}

impl Named for MultiTestCase {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Testable for MultiTestCase {
    fn start(&self) -> Box<dyn crate::PendingReport> {
        todo!()
    }
}

impl TestCase for MultiTestCase {}

impl MultiTestCase {
    pub fn new(name: impl Into<String>, children: Vec<impl Into<NodeTestCase>>) -> Self {
        let children = children.into_iter().map(|x| x.into()).collect();
        let name: String = name.into();
        Self { name, children }
    }
}

impl Ignorable for &NodeTestCase {}

impl Named for &NodeTestCase {
    fn name(&self) -> &str {
        match self {
            NodeTestCase::Single(x) => x.name(),
            NodeTestCase::Multi(x) => x.name(),
        }
    }
}

impl Testable for &NodeTestCase {
    fn start(&self) -> Box<dyn crate::PendingReport> {
        match self {
            NodeTestCase::Single(x) => x.start(),
            NodeTestCase::Multi(x) => x.start(),
        }
    }
}

impl TestCase for &NodeTestCase {}

impl From<SingleTestCase> for NodeTestCase {
    fn from(x: SingleTestCase) -> Self {
        NodeTestCase::Single(x)
    }
}

impl From<MultiTestCase> for NodeTestCase {
    fn from(x: MultiTestCase) -> Self {
        NodeTestCase::Multi(x)
    }
}

#[cfg(test)]
mod tests {
    use crate::TestResult;
    use chrono::Duration;

    use super::*;

    #[derive(Debug)]
    struct MockRunner;

    impl Runner for MockRunner {
        fn start(&self) -> Box<dyn crate::PendingReport> {
            Box::new(
                crate::pending_report::ImmediatelyReadyPendingReport::from_result(
                    TestResult::Pass,
                    Duration::seconds(1),
                ),
            )
        }
    }

    #[test]
    fn test_single_test_case() {
        let test_case = SingleTestCase::new("one", Box::new(MockRunner));
        assert_eq!("one", test_case.name());
        let mut report = test_case.start();
        let report = report.poll();
        let report = report.unwrap();
        assert_eq!(TestResult::Pass, report.result());
    }

    #[test]
    fn test_test_case_with_children() {
        let children = vec![
            SingleTestCase::new("one", Box::new(MockRunner)),
            SingleTestCase::new("two", Box::new(MockRunner)),
            SingleTestCase::new("three", Box::new(MockRunner)),
        ];

        let test_case = MultiTestCase::new("group", children);
        assert_eq!("group", test_case.name());
        // TODO
        // let mut report = test_case.start();
        // let _report = report.poll();
    }
}
