use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, ResetEvent};
use crate::model::habit::HabitDb;

pub struct ResetAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl ResetAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn reset_all(&self) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::Reset(ResetEvent::Resetting));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::Reset(ResetEvent::Failed(e.to_string())));
                    return;
                }
            };
            match habit_db.execute_batch("DELETE FROM habit_log; DELETE FROM habit;") {
                Ok(()) => {
                    let _ = sender.send(AppEvent::Reset(ResetEvent::Reset));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::Reset(ResetEvent::Failed(e.to_string())));
                }
            }
        });
    }
}
