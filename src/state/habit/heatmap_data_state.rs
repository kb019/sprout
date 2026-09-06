use std::collections::HashSet;

#[derive(Default)]
pub struct HeatmapDataState {
    fetching: HashSet<(i32, i32)>,
}

impl HeatmapDataState {
    pub fn new() -> Self {
        Self {
            fetching: HashSet::new(),
        }
    }

    pub fn start_fetching(&mut self, habit_id: i32, year: i32) {
        self.fetching.insert((habit_id, year));
    }

    pub fn stop_fetching(&mut self, habit_id: i32, year: i32) {
        self.fetching.remove(&(habit_id, year));
    }

    pub fn is_fetching(&self, habit_id: i32, year: i32) -> bool {
        self.fetching.contains(&(habit_id, year))
    }

    pub fn is_any_fetching(&self) -> bool {
        !self.fetching.is_empty()
    }
}
