use super::report::Report;

trait Testable {
    fn run() -> Report;
}