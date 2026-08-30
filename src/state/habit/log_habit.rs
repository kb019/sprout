use std::collections::HashSet;

#[derive(Default)]
pub struct LogHabitState {
    logging_habit_ids: HashSet<i32>,
}

impl LogHabitState {
    pub fn new() -> Self {
        Self {
            logging_habit_ids: HashSet::new(),
        }
    }

    pub fn start_logging(&mut self, habit_id: i32) {
        self.logging_habit_ids.insert(habit_id);
    }

    pub fn stop_logging(&mut self, habit_id: i32) {
        self.logging_habit_ids.remove(&habit_id);
    }

    pub fn is_habit_logging(&self, habit_id: i32) -> bool {
        self.logging_habit_ids.contains(&habit_id)
    }
}
