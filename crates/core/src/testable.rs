use crate::pending_report::PendingReport;

pub trait Runner: core::fmt::Debug {
    fn start(&self) -> Box<dyn PendingReport>;
}

pub trait Testable {
    fn start(&self) -> Box<dyn PendingReport>;
}
