mod add_habit;

use std::path::PathBuf;
use std::sync::mpsc;

use crate::event::AppEvent;
use crate::model::habit::NewHabit;
use add_habit::AddHabitAction;

pub struct Actions {
    add_habit_action: AddHabitAction,
}

impl Actions {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self {
            add_habit_action: AddHabitAction::new(sender, db_path),
        }
    }

    pub fn add_habit(&self, new_habit: NewHabit) {
        self.add_habit_action.add_habit(new_habit);
    }
}
