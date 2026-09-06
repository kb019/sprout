use ratatui::widgets::ListState;

pub struct ResetModalState {
    button_state: ListState,
    is_resetting: bool,
}

impl ResetModalState {
    pub fn new() -> Self {
        let mut state = Self {
            button_state: ListState::default(),
            is_resetting: false,
        };
        state.button_state.select(Some(0));
        state
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn button_state_mut(&mut self) -> &mut ListState {
        &mut self.button_state
    }

    pub fn selected_button(&self) -> usize {
        self.button_state.selected().unwrap_or(0)
    }

    pub fn next_button(&mut self) {
        let next = self
            .button_state
            .selected()
            .map(|i| (i + 1).min(1))
            .unwrap_or(1);
        self.button_state.select(Some(next));
    }

    pub fn prev_button(&mut self) {
        let prev = self
            .button_state
            .selected()
            .map(|i| i.saturating_sub(1))
            .unwrap_or(0);
        self.button_state.select(Some(prev));
    }

    pub fn is_resetting(&self) -> bool {
        self.is_resetting
    }

    pub fn set_is_resetting(&mut self, value: bool) {
        self.is_resetting = value;
    }
}

impl Default for ResetModalState {
    fn default() -> Self {
        Self::new()
    }
}
