use crate::report::Report;
use super::testable::Testable;

pub struct Collection {
    items: Vec<Box<dyn Testable>>,
    ignored: bool,
}

impl Testable for Collection {
    fn run(&self) -> Report {
        if self.ignored {
            return Report::skipped();
        }

        let children = self.items.iter().map(|t| t.run()).collect();

        Report::with_children(children)
    }

    fn is_ignored(&self) -> bool {
        self.ignored
    }
}