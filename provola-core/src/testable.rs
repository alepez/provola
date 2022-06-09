use super::report::Report;
use super::error::Error;

pub trait Testable {
    fn run(&self) -> Report;
    fn is_ignored(&self) -> bool;
}