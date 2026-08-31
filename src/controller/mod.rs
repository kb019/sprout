mod add_habit;
mod delete_habit;
mod get_streak;
mod log_habit;

use std::path::PathBuf;
use std::sync::mpsc;

use crate::event::AppEvent;
use crate::model::habit::NewHabit;
use add_habit::AddHabitAction;
use delete_habit::DeleteHabitAction;
use get_streak::GetStreakAction;
use log_habit::LogHabitAction;

pub struct Actions {
    add_habit_action: AddHabitAction,
    log_habit_action: LogHabitAction,
    delete_habit_action: DeleteHabitAction,
    get_streak_action: GetStreakAction,
}

impl Actions {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self {
            add_habit_action: AddHabitAction::new(sender.clone(), db_path.clone()),
            log_habit_action: LogHabitAction::new(sender.clone(), db_path.clone()),
            delete_habit_action: DeleteHabitAction::new(sender.clone(), db_path.clone()),
            get_streak_action: GetStreakAction::new(sender, db_path),
        }
    }

    pub fn add_habit(&self, new_habit: NewHabit) {
        self.add_habit_action.add_habit(new_habit);
    }

    pub fn log_habit(&self, habit_id: i32, completed: i32, progress: i32) {
        self.log_habit_action
            .log_habit(habit_id, completed, progress);
    }

    pub fn delete_habit(&self, habit_id: i32) {
        self.delete_habit_action.delete_habit(habit_id);
    }

    pub fn get_streak(&self, habit_id: i32) {
        self.get_streak_action.get_streak(habit_id);
    }
}
