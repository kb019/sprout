use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, EditHabitEvent};
use crate::model::habit::{HabitDb, HabitUpdate};

pub struct EditHabitAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl EditHabitAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn edit_habit(&self, update: HabitUpdate, created_at: String) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let habit_id = update.id;
        let _ = sender.send(AppEvent::EditHabit(EditHabitEvent::Editing(habit_id)));
        thread::spawn(move || {
            let mut habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::EditHabit(EditHabitEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                    return;
                }
            };
            match habit_db.update_habit(&update, created_at) {
                Ok(habit) => {
                    let _ = sender.send(AppEvent::EditHabit(EditHabitEvent::Edited(habit)));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::EditHabit(EditHabitEvent::Failed(
                        habit_id,
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
