use crate::pending_report::PendingReport;

pub trait Testable: Send {
    fn start(&self) -> Box<dyn PendingReport>;
}
