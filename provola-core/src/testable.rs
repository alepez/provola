use super::report::Report;
use super::error::Error;

pub trait Testable {
    fn run(&self) -> Result<Report, Error>;
    fn is_ignored(&self) -> bool;
}