use crate::testable::Testable;

pub trait Ignorable {
    fn is_ignored(&self) -> bool;
    fn set_ignored(&self) -> Result<(), ()>;
}

pub trait TestCase: Ignorable + Testable {}
