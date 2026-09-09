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
