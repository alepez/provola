use crate::testable::Testable;

pub struct TestCase {
    name: String,
    runner: Box<dyn Testable>,
}