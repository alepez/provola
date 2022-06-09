use super::report::Report;
use super::error::Error;

pub trait Testable {
    fn run(&self) -> Result<Report, Error>;
}