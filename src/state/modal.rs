use ratatui::widgets::ListState;

use crate::state::modal::add_modal::AddModalState;
mod add_modal;
pub struct ModalState {
    button_state: ListState,
    add_modal_state: AddModalState,
}

impl ModalState {
    pub fn new() -> Self {
        let mut button_state = ListState::default();
        button_state.select(Some(0));
        Self {
            button_state,
            add_modal_state: AddModalState::new(),
        }
    }

    pub fn button_state_mut(&mut self) -> &mut ListState {
        &mut self.button_state
    }

    pub fn add_modal_state_mut(&mut self) -> &mut AddModalState {
        &mut self.add_modal_state
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

    pub fn reset(&mut self) {
        self.button_state.select(Some(0));
        self.add_modal_state.reset();
    }
}

impl Default for ModalState {
    fn default() -> Self {
        Self::new()
    }
}
