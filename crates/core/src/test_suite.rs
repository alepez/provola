use crate::{Ignorable, Named, TestCase, Testable};

#[derive(Debug, Clone)]
pub struct SingleTestCase {
    name: String,
}

#[derive(Debug, Clone)]
pub struct MultiTestCase {
    name: String,
    #[allow(dead_code)]
    children: Vec<NodeTestCase>,
}

#[derive(Debug, Clone)]
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
        todo!()
    }
}

impl TestCase for SingleTestCase {}

impl SingleTestCase {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        Self { name }
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
    use super::*;

    #[test]
    fn test_single_test_case() {
        let test_case = SingleTestCase::new("one");
        assert_eq!("one", test_case.name());
    }

    #[test]
    fn test_test_case_with_children() {
        let children = vec![
            SingleTestCase::new("one"),
            SingleTestCase::new("two"),
            SingleTestCase::new("three"),
        ];

        let test_case = MultiTestCase::new("group", children);
        assert_eq!("group", test_case.name());
    }
}
