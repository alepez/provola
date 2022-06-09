use super::report::Report;

pub trait Testable {
    fn run(&self) -> Report {
        Report::skipped()
    }

    fn is_ignored(&self) -> bool {
        false
    }
}