use crate::state::modal::add_modal::AddModalState;
mod add_modal;

pub struct ModalState {
    add_modal_state: AddModalState,
}

impl ModalState {
    pub fn new() -> Self {
        Self {
            add_modal_state: AddModalState::new(),
        }
    }

    pub fn add_modal_state_mut(&mut self) -> &mut AddModalState {
        &mut self.add_modal_state
    }

    pub fn reset(&mut self) {
        self.add_modal_state.reset();
    }
}

impl Default for ModalState {
    fn default() -> Self {
        Self::new()
    }
}
