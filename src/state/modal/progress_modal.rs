use ratatui::widgets::ListState;

use crate::state::input::{InputState, InputType};

pub struct ProgressModalState {
    pub progress_input_state: InputState,
    button_state: ListState, // 0 = Add, 1 = Cancel
}

impl ProgressModalState {
    pub fn new() -> Self {
        let mut state = Self {
            progress_input_state: InputState::new(),
            button_state: ListState::default(),
        };

        state.progress_input_state.set_input_type(InputType::Number);
        state.progress_input_state.set_max_length(10);
        state.progress_input_state.set_focus(true);
        state.button_state.select(Some(0)); // Add button selected initially
        state
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn progress_input_state_mut(&mut self) -> &mut InputState {
        &mut self.progress_input_state
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
            .unwrap_or(0);
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
}
