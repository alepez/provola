use super::testable::Testable;
use crate::report::{Report, TestResult};

pub struct Collection {
    items: Vec<Box<dyn Testable>>,
    ignored: bool,
}

impl Testable for Collection {
    fn start(&self) -> Report {
        if self.ignored {
            return Report::skipped();
        }

        if self.items.is_empty() {
            return Report {
                result: TestResult::Empty,
                duration: None,
                children: vec![],
            };
        }

        let children = self.items.iter().map(|t| t.start()).collect();

        Report::with_children(children)
    }

    fn is_ignored(&self) -> bool {
        self.ignored
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::TestResult;

    #[test]
    fn ignored_collection_shall_give_skipped_as_result() {
        let c = Collection {
            items: Default::default(),
            ignored: true,
        };
        let r = c.start();
        assert!(matches!(r.result, TestResult::Skipped));
    }

    #[test]
    fn collection_without_children() {
        let c = Collection {
            items: Default::default(),
            ignored: false,
        };
        let r = c.start();
        assert!(matches!(dbg!(&r.result), TestResult::Empty));
    }
}
