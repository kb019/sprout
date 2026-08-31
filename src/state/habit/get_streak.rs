use std::collections::HashSet;

#[derive(Default)]
pub struct GetStreakState {
    fetching_habit_ids: HashSet<i32>,
}

impl GetStreakState {
    pub fn new() -> Self {
        Self {
            fetching_habit_ids: HashSet::new(),
        }
    }

    pub fn start_fetching(&mut self, habit_id: i32) {
        self.fetching_habit_ids.insert(habit_id);
    }

    pub fn stop_fetching(&mut self, habit_id: i32) {
        self.fetching_habit_ids.remove(&habit_id);
    }

    pub fn is_fetching(&self, habit_id: i32) -> bool {
        self.fetching_habit_ids.contains(&habit_id)
    }

    pub fn fetching_ids(&self) -> &HashSet<i32> {
        &self.fetching_habit_ids
    }
}
