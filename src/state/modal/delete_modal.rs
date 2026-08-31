use ratatui::widgets::ListState;

pub struct DeleteModalState {
    button_state: ListState, // 0 = Confirm, 1 = Cancel
    is_deleting: bool,
}

impl DeleteModalState {
    pub fn new() -> Self {
        let mut state = Self {
            button_state: ListState::default(),
            is_deleting: false,
        };
        state.button_state.select(Some(0)); // Default to Confirm
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

    pub fn is_deleting(&self) -> bool {
        self.is_deleting
    }

    pub fn set_is_deleting(&mut self, value: bool) {
        self.is_deleting = value;
    }
}
