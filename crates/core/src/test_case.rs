use crate::{error::Error, testable::Testable, Named};

pub trait Ignorable {
    fn is_ignored(&self) -> bool {
        // Not ignored by default
        false
    }

    fn set_ignored(&self) -> Result<(), Error> {
        // Usually we cannot change the ignored flag
        Err(Error::NotAvailable)
    }
}

pub trait TestCase: Ignorable + Testable + Named {}
