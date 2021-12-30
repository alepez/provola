use crate::{ActionConfig, GuiConfig, ProvolaGuiApp};
use crate::*;
use crossbeam_channel::select;
use provola_core::{Action, Error, WatchOptions, Watcher};
use std::{path::PathBuf, thread, time::Duration};

pub(crate) struct Server {
    pub opt: Option<GuiConfig>,
    pub action_r: ActionReceiver,
    pub feedback_s: FeedbackSender,
}

impl Server {
    fn handle_message(&mut self, msg: ActionMessage) {
        match msg {
            ActionMessage::Setup(setup) => {
                // This must be done before starting watch thread
                self.feedback_s.repaint_signal = Some(setup.repaint_signal);

                if let Some(watch_path) = &setup.config.watch_path {
                    // FIXME Make this thread stoppable (when watch_path changes)
                    // FIXME Start this thread even if watch_path is currently None
                    self.start_watch_thread(watch_path.clone());
                }

                self.opt = Some(setup.config);

                self.handle_result(self.get_available_tests());
            }
            ActionMessage::RunAll => {
                self.handle_result(self.run_once());
            }
            ActionMessage::UpdateConfig(new_config) => {
                log::debug!("Configuration changed");
                self.opt = Some(new_config);
            }
            ActionMessage::ReqAvailableTests => {
                self.handle_result(self.get_available_tests());
            }
        }
    }

    fn handle_result<T>(&self, res: Result<T, Error>) {
        res.map_err(|e| self.send_error_feedback(e)).ok();
    }

    fn send_error_feedback(&self, err: impl ToString) {
        self.feedback_s
            .send(FeedbackMessage::Error(err.to_string()));
    }

    pub(crate) fn run(&mut self) {
        loop {
            select! {
                recv(self.action_r) -> msg => {
                    match msg {
                        Ok(msg) => self.handle_message(msg),
                        Err(_) => return,
                    }
                },
            }
        }
    }

    fn start_watch_thread(&self, w: PathBuf) {
        let feedback_s = self.feedback_s.clone();

        feedback_s.send(FeedbackMessage::WatchedChanged);

        thread::spawn(move || {
            let watch_opt = WatchOptions {
                file: w,
                debounce_time: Duration::from_secs(1),
            };

            // TODO watch must be stopped when file_to_watch changes
            Watcher::try_from(watch_opt)
                .unwrap()
                .watch(&mut || {
                    log::debug!("Watched change detected");
                    feedback_s.send(FeedbackMessage::WatchedChanged);
                })
                .unwrap();
        });
    }

    fn run_once(&self) -> Result<(), Error> {
        let opt = self.opt.as_ref().ok_or(Error::NoResult)?;

        let action = Action::try_from(opt)?;
        let result = action.run()?;

        self.feedback_s.send(FeedbackMessage::Result(result));

        Ok(())
    }

    fn get_available_tests(&self) -> Result<(), Error> {
        let opt = self.opt.as_ref().ok_or(Error::NoResult)?;

        let action = Action::try_from(opt)?;

        let list = match action {
            Action::TestRunner(tr, opt) => Ok(tr.list(&opt)?),
            _ => Err(Error::NothingToDo),
        }?;

        self.feedback_s.send(FeedbackMessage::AvailableTests(list));

        Ok(())
    }
}
