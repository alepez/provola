use super::report::Report;

pub trait Testable {
    fn run(&self) -> Report;
    fn is_ignored(&self) -> bool;
}