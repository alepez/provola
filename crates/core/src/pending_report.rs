use crate::report::Report;

pub trait PendingReport {
    fn poll(&mut self) -> Option<Box<dyn Report>>;
}
