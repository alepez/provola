use crate::Error;
use notify::{watcher, DebouncedEvent, INotifyWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct WatchOptions {
    pub file: PathBuf,
    pub debounce_time: Duration,
}

pub struct ProvolaWatcher {
    rx: Receiver<DebouncedEvent>,
    _w: INotifyWatcher,
}

impl TryFrom<WatchOptions> for ProvolaWatcher {
    type Error = Error;

    fn try_from(opt: WatchOptions) -> Result<Self, Error> {
        let (tx, rx) = channel();

        let mut w =
            watcher(tx, opt.debounce_time).map_err(|e| Error::CannotWatch(e.to_string()))?;

        w.watch(&opt.file, RecursiveMode::Recursive)
            .map_err(|e| Error::CannotWatch(e.to_string()))?;

        Ok(ProvolaWatcher { rx, _w: w })
    }
}

impl ProvolaWatcher {
    pub fn rx(&self) -> &Receiver<DebouncedEvent> {
        &self.rx
    }

    pub fn watch(self, f: &mut dyn FnMut()) -> Result<(), Error> {
        let rx = self.rx();

        loop {
            match rx.recv() {
                Ok(_) => {
                    f();
                }
                Err(e) => {
                    return Err(Error::CannotWatch(e.to_string()));
                }
            }
        }
    }
}
