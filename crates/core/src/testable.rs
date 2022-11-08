use crate::report::PendingReport;

pub trait Testable: Send {
    fn start(&self) -> Box<dyn PendingReport>;

    fn is_ignored(&self) -> bool;
}
