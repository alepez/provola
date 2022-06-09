use super::report::Report;

pub trait Testable {
    fn run(&self) -> Report {
        Report::not_available()
    }

    fn is_ignored(&self) -> bool {
        false
    }
}