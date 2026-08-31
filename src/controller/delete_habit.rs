use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, DeleteHabitEvent};
use crate::model::habit::HabitDb;

pub struct DeleteHabitAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl DeleteHabitAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn delete_habit(&self, habit_id: i32) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::DeleteHabit(DeleteHabitEvent::Deleting(habit_id)));
        thread::spawn(move || {
            let mut habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::DeleteHabit(DeleteHabitEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.delete_habit(habit_id) {
                Ok(()) => {
                    let _ = sender.send(AppEvent::DeleteHabit(DeleteHabitEvent::Deleted(habit_id)));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::DeleteHabit(DeleteHabitEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
