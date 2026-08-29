use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AddHabitEvent, AppEvent};
use crate::model::habit::{HabitDb, NewHabit};

pub struct AddHabitAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl AddHabitAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn add_habit(&self, new_habit: NewHabit) {
        let sender = self.sender.clone();
        let path = self.db_path.clone();
        let _ = sender.send(AppEvent::AddHabit(AddHabitEvent::Adding));
        thread::spawn(move || {
            let habit_db = match HabitDb::new(&path) {
                Ok(h) => h,
                Err(e) => {
                    let _ = sender.send(AppEvent::AddHabit(AddHabitEvent::Failed(e.to_string())));
                    return;
                }
            };
            match habit_db.create_habit(&new_habit) {
                Ok(habit) => {
                    let _ = sender.send(AppEvent::AddHabit(AddHabitEvent::Added(habit)));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::AddHabit(AddHabitEvent::Failed(e.to_string())));
                }
            }
        });
    }
}
