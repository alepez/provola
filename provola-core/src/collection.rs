use std::ops::Index;
use crate::error::Error;
use crate::report::Report;
use super::testable::Testable;

pub struct Collection {
    items: Vec<Box<dyn Testable>>,
}

impl Testable for Collection {
    fn run(&self) -> Result<Report, Error> {
        todo!()
    }

    fn is_ignored(&self) -> bool {
        todo!()
    }
}