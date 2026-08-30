use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, LogHabitEvent};
use crate::model::habit::HabitDb;

pub struct LogHabitAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl LogHabitAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn log_habit(&self, habit_id: i32, progress: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::LogHabit(LogHabitEvent::Logging(habit_id)));
        thread::spawn(move || {
            let mut habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::LogHabit(LogHabitEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.log_habit_progress(habit_id, progress) {
                Ok(log) => {
                    // thread::sleep(std::time::Duration::from_millis(5000));
                    let _ = sender.send(AppEvent::LogHabit(LogHabitEvent::Logged(log)));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::LogHabit(LogHabitEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
