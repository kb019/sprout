#[derive(Default)]
pub struct WeeklyAverageState {
    is_fetching: bool,
}

impl WeeklyAverageState {
    pub fn new() -> Self {
        Self { is_fetching: false }
    }

    pub fn start_fetching(&mut self) {
        self.is_fetching = true;
    }

    pub fn stop_fetching(&mut self) {
        self.is_fetching = false;
    }

    pub fn is_fetching(&self) -> bool {
        self.is_fetching
    }
}
