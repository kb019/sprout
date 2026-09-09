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

    pub fn is_deleting(&self) -> bool {
        self.is_deleting
    }

    pub fn set_is_deleting(&mut self, value: bool) {
        self.is_deleting = value;
    }
}
