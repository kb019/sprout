use ratatui::widgets::ListState;
use std::collections::HashMap;

pub struct AppState {
    menu_state: ListState,
    heatmap_tile_states: HashMap<i32, ListState>,
    settings_state: ListState,
    settings_tile_states: Vec<ListState>,
    dashboard_habits_state: ListState,
    best_streaks_list_state: ListState,
    streak_leaderboard_state: ListState,
    year_list_state: ListState,
    goal_progress_tile_state: ListState,
    goal_progress_row_state: Vec<ListState>,
}

impl AppState {
    pub fn new() -> Self {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));
        let mut settings_state = ListState::default();
        settings_state.select(None);
        let mut settings_tile_states: Vec<ListState> =
            (0..5).map(|_| ListState::default()).collect();
        for s in &mut settings_tile_states {
            s.select(Some(0));
        }
        let mut goal_progress_tile_state = ListState::default();
        goal_progress_tile_state.select(Some(0));
        // 0=daily, 1=weekly, 2=monthly, 3=yearly
        let goal_progress_row_state: Vec<ListState> =
            (0..4).map(|_| ListState::default()).collect();
        let mut year_list_state = ListState::default();
        year_list_state.select(Some(0));
        Self {
            menu_state,
            heatmap_tile_states: HashMap::new(),
            settings_state,
            settings_tile_states,
            dashboard_habits_state: ListState::default(),
            best_streaks_list_state: ListState::default(),
            streak_leaderboard_state: ListState::default(),
            year_list_state,
            goal_progress_tile_state,
            goal_progress_row_state,
        }
    }

    pub fn menu_state(&self) -> &ListState {
        &self.menu_state
    }

    pub fn menu_state_mut(&mut self) -> &mut ListState {
        &mut self.menu_state
    }

    /// Insert a state for each year that doesn't already have one.
    /// Existing selections are never touched.
    pub fn update_heatmap_tile_states(&mut self, year_habits: &[(i32, Vec<i32>)]) {
        for (year, _) in year_habits {
            self.heatmap_tile_states.entry(*year).or_insert_with(|| {
                let mut s = ListState::default();
                s.select(Some(0));
                s
            });
        }
    }

    pub fn heatmap_tile_state_for_year_mut(&mut self, year: i32) -> Option<&mut ListState> {
        self.heatmap_tile_states.get_mut(&year)
    }

    pub fn settings_state(&self) -> &ListState {
        &self.settings_state
    }

    pub fn settings_state_mut(&mut self) -> &mut ListState {
        &mut self.settings_state
    }

    pub fn settings_states_mut(&mut self) -> (&mut ListState, &mut Vec<ListState>) {
        (&mut self.settings_state, &mut self.settings_tile_states)
    }

    pub fn settings_tile_selected(&self, row: usize) -> Option<usize> {
        self.settings_tile_states
            .get(row)
            .and_then(|s| s.selected())
    }

    pub fn active_theme(&self) -> usize {
        self.settings_tile_states
            .first()
            .and_then(|s| s.selected())
            .unwrap_or(0)
    }

    pub fn next_settings_tile(&mut self, row: usize, len: usize) {
        if let Some(state) = self.settings_tile_states.get_mut(row) {
            let next = state.selected().map(|i| (i + 1).min(len - 1)).unwrap_or(0);
            state.select(Some(next));
        }
    }

    pub fn prev_settings_tile(&mut self, row: usize, _len: usize) {
        if let Some(state) = self.settings_tile_states.get_mut(row) {
            let prev = state.selected().map(|i| i.saturating_sub(1)).unwrap_or(0);
            state.select(Some(prev));
        }
    }

    pub fn next_menu(&mut self, len: usize) {
        let next = self
            .menu_state
            .selected()
            .map(|i| (i + 1).min(len - 1))
            .unwrap_or(0);
        self.menu_state.select(Some(next));
    }

    pub fn prev_menu(&mut self, _len: usize) {
        let prev = self
            .menu_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.menu_state.select(Some(prev));
    }

    pub fn next_heatmap_tile(&mut self, year: i32, len: usize) {
        if len == 0 {
            return;
        }
        if let Some(state) = self.heatmap_tile_states.get_mut(&year) {
            let next = state.selected().map(|i| (i + 1).min(len - 1)).unwrap_or(0);
            state.select(Some(next));
        }
    }

    pub fn prev_heatmap_tile(&mut self, year: i32) {
        if let Some(state) = self.heatmap_tile_states.get_mut(&year) {
            let prev = state.selected().map(|i| i.saturating_sub(1)).unwrap_or(0);
            state.select(Some(prev));
        }
    }

    pub fn next_settings(&mut self, len: usize) {
        let next = self
            .settings_state
            .selected()
            .map(|i| (i + 1).min(len - 1))
            .unwrap_or(0);
        self.settings_state.select(Some(next));
    }

    pub fn prev_settings(&mut self, _len: usize) {
        let prev = self
            .settings_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.settings_state.select(Some(prev));
    }

    pub fn clear_settings(&mut self) {
        self.settings_state.select(None);
    }

    pub fn dashboard_habits_state(&self) -> &ListState {
        &self.dashboard_habits_state
    }

    pub fn dashboard_habits_state_mut(&mut self) -> &mut ListState {
        &mut self.dashboard_habits_state
    }

    pub fn next_dashboard_habit(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let next = self
            .dashboard_habits_state
            .selected()
            .map(|i| (i + 1).min(len - 1))
            .unwrap_or(0);
        self.dashboard_habits_state.select(Some(next));
    }

    pub fn prev_dashboard_habit(&mut self, _len: usize) {
        let prev = self
            .dashboard_habits_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.dashboard_habits_state.select(Some(prev));
    }

    pub fn best_streaks_list_state_mut(&mut self) -> &mut ListState {
        &mut self.best_streaks_list_state
    }

    pub fn next_best_streak(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let next = self
            .best_streaks_list_state
            .selected()
            .map(|i| (i + 1).min(len - 1))
            .unwrap_or(0);
        self.best_streaks_list_state.select(Some(next));
    }

    pub fn prev_best_streak(&mut self) {
        let prev = self
            .best_streaks_list_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.best_streaks_list_state.select(Some(prev));
    }

    pub fn streak_leaderboard_state_mut(&mut self) -> &mut ListState {
        &mut self.streak_leaderboard_state
    }

    pub fn year_list_state(&self) -> &ListState {
        &self.year_list_state
    }

    pub fn year_list_state_mut(&mut self) -> &mut ListState {
        &mut self.year_list_state
    }

    pub fn next_year(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let next = self
            .year_list_state
            .selected()
            .map(|i| (i + 1).min(len - 1))
            .unwrap_or(0);
        self.year_list_state.select(Some(next));
    }

    pub fn prev_year(&mut self) {
        let prev = self
            .year_list_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.year_list_state.select(Some(prev));
    }

    pub fn next_streak_leaderboard(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let next = self
            .streak_leaderboard_state
            .selected()
            .map(|i| (i + 1).min(len - 1))
            .unwrap_or(0);
        self.streak_leaderboard_state.select(Some(next));
    }

    pub fn prev_streak_leaderboard(&mut self) {
        let prev = self
            .streak_leaderboard_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.streak_leaderboard_state.select(Some(prev));
    }

    pub fn goal_progress_tile_state(&self) -> &ListState {
        &self.goal_progress_tile_state
    }

    pub fn goal_progress_tile_state_mut(&mut self) -> &mut ListState {
        &mut self.goal_progress_tile_state
    }

    pub fn next_goal_progress(&mut self, len: usize) {
        let next = self
            .goal_progress_tile_state
            .selected()
            .map(|i| (i + 1).min(len - 1))
            .unwrap_or(0);
        self.goal_progress_tile_state.select(Some(next));
    }

    pub fn prev_goal_progress(&mut self) {
        let prev = self
            .goal_progress_tile_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.goal_progress_tile_state.select(Some(prev));
    }

    pub fn goal_progress_states_mut(&mut self) -> (&mut ListState, &mut Vec<ListState>) {
        (
            &mut self.goal_progress_tile_state,
            &mut self.goal_progress_row_state,
        )
    }

    pub fn next_goal_progress_row(&mut self, tab: usize, len: usize) {
        if let Some(state) = self.goal_progress_row_state.get_mut(tab) {
            let next = state.selected().map(|i| (i + 1).min(len - 1)).unwrap_or(0);
            state.select(Some(next));
        }
    }

    pub fn prev_goal_progress_row(&mut self, tab: usize) {
        if let Some(state) = self.goal_progress_row_state.get_mut(tab) {
            let prev = state.selected().map(|i| i.saturating_sub(1)).unwrap_or(0);
            state.select(Some(prev));
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
