use ratatui::widgets::ListState;

pub struct State {
    menu_state: ListState,
    heatmap_tile_state: ListState,
    settings_state: ListState,
    settings_tile_states: Vec<ListState>,
}

impl State {
    pub fn new() -> Self {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));
        let mut heatmap_tile_state = ListState::default();
        heatmap_tile_state.select(Some(0));
        let mut settings_state = ListState::default();
        settings_state.select(None);
        let mut settings_tile_states: Vec<ListState> =
            (0..3).map(|_| ListState::default()).collect();
        for s in &mut settings_tile_states {
            s.select(Some(0));
        }
        Self {
            menu_state,
            heatmap_tile_state,
            settings_state,
            settings_tile_states,
        }
    }

    pub fn menu_state(&self) -> &ListState {
        &self.menu_state
    }

    pub fn heatmap_tile_state(&self) -> &ListState {
        &self.heatmap_tile_state
    }

    pub fn settings_state(&self) -> &ListState {
        &self.settings_state
    }

    pub fn menu_state_mut(&mut self) -> &mut ListState {
        &mut self.menu_state
    }

    pub fn heatmap_tile_state_mut(&mut self) -> &mut ListState {
        &mut self.heatmap_tile_state
    }

    pub fn settings_state_mut(&mut self) -> &mut ListState {
        &mut self.settings_state
    }

    pub fn settings_states_mut(&mut self) -> (&mut ListState, &mut Vec<ListState>) {
        (&mut self.settings_state, &mut self.settings_tile_states)
    }

    pub fn active_theme(&self) -> usize {
        self.settings_tile_states
            .first()
            .and_then(|s| s.selected())
            .unwrap_or(0)
    }

    pub fn next_settings_tile(&mut self, row: usize, len: usize) {
        if let Some(state) = self.settings_tile_states.get_mut(row) {
            let next = state.selected().map(|i| (i + 1) % len).unwrap_or(0);
            state.select(Some(next));
        }
    }

    pub fn prev_settings_tile(&mut self, row: usize, len: usize) {
        if let Some(state) = self.settings_tile_states.get_mut(row) {
            let prev = state
                .selected()
                .map(|i| if i == 0 { len - 1 } else { i - 1 })
                .unwrap_or(0);
            state.select(Some(prev));
        }
    }

    pub fn next_menu(&mut self, len: usize) {
        let next = self
            .menu_state
            .selected()
            .map(|i| (i + 1) % len)
            .unwrap_or(0);
        self.menu_state.select(Some(next));
    }

    pub fn prev_menu(&mut self, len: usize) {
        let prev = self
            .menu_state
            .selected()
            .map(|i| if i == 0 { len - 1 } else { i - 1 })
            .unwrap_or(0);
        self.menu_state.select(Some(prev));
    }

    pub fn next_heatmap_tile(&mut self, len: usize) {
        let next = self
            .heatmap_tile_state
            .selected()
            .map(|i| (i + 1) % len)
            .unwrap_or(0);
        self.heatmap_tile_state.select(Some(next));
    }

    pub fn prev_heatmap_tile(&mut self, len: usize) {
        let prev = self
            .heatmap_tile_state
            .selected()
            .map(|i| if i == 0 { len - 1 } else { i - 1 })
            .unwrap_or(0);
        self.heatmap_tile_state.select(Some(prev));
    }

    pub fn next_settings(&mut self, len: usize) {
        let next = self
            .settings_state
            .selected()
            .map(|i| (i + 1) % len)
            .unwrap_or(0);
        self.settings_state.select(Some(next));
    }

    pub fn prev_settings(&mut self, len: usize) {
        let prev = self
            .settings_state
            .selected()
            .map(|i| if i == 0 { len - 1 } else { i - 1 })
            .unwrap_or(0);
        self.settings_state.select(Some(prev));
    }

    pub fn clear_settings(&mut self) {
        self.settings_state.select(None);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
