use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
};
use crate::report::Report;
use crate::Testable;

#[derive(Default)]
pub struct ReportFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

#[derive(Default)]
struct SharedState {
    report: Option<Report>,
    waker: Option<Waker>,
}


impl Future for ReportFuture {
    type Output = Report;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if let Some(report) = shared_state.report.take() {
            Poll::Ready(report)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl ReportFuture {
    pub fn new(testable: Arc<Mutex<Box<dyn Testable>>>) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            report: None,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        thread::spawn(move || {
            let testable = testable.lock().unwrap();
            let report = testable.run();
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.report = Some(report);
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        ReportFuture { shared_state }
    }
}
