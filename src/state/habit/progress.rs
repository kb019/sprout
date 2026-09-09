use std::collections::HashSet;

#[derive(Default)]
pub struct DailyProgressState {
    fetching_habit_ids: HashSet<i32>,
}

impl DailyProgressState {
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
}

#[derive(Default)]
pub struct WeeklyProgressState {
    fetching_habit_ids: HashSet<i32>,
}

impl WeeklyProgressState {
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
}

#[derive(Default)]
pub struct MonthlyProgressState {
    fetching_habit_ids: HashSet<i32>,
}

impl MonthlyProgressState {
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
}

#[derive(Default)]
pub struct YearlyProgressState {
    fetching_habit_ids: HashSet<i32>,
}

impl YearlyProgressState {
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
}
