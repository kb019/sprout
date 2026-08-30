use crate::state::modal::{add_modal::AddModalState, progress_modal::ProgressModalState};
mod add_modal;
mod progress_modal;

pub struct ModalState {
    add_modal_state: AddModalState,
    progress_modal_state: ProgressModalState,
}

impl ModalState {
    pub fn new() -> Self {
        Self {
            add_modal_state: AddModalState::new(),
            progress_modal_state: ProgressModalState::new(),
        }
    }

    pub fn add_modal_state_mut(&mut self) -> &mut AddModalState {
        &mut self.add_modal_state
    }

    pub fn progress_modal_state_mut(&mut self) -> &mut ProgressModalState {
        &mut self.progress_modal_state
    }

    pub fn reset(&mut self) {
        self.add_modal_state.reset();
        self.progress_modal_state.reset();
    }
}

impl Default for ModalState {
    fn default() -> Self {
        Self::new()
    }
}
