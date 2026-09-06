use crate::state::modal::{
    add_modal::AddModalState, delete_modal::DeleteModalState, edit_modal::EditModalState,
    progress_modal::ProgressModalState, reset_modal::ResetModalState,
};
mod add_modal;
mod delete_modal;
mod edit_modal;
mod progress_modal;
mod reset_modal;

pub struct ModalState {
    add_modal_state: AddModalState,
    edit_modal_state: EditModalState,
    progress_modal_state: ProgressModalState,
    delete_modal_state: DeleteModalState,
    reset_modal_state: ResetModalState,
}

impl ModalState {
    pub fn new() -> Self {
        Self {
            add_modal_state: AddModalState::new(),
            edit_modal_state: EditModalState::new(),
            progress_modal_state: ProgressModalState::new(),
            delete_modal_state: DeleteModalState::new(),
            reset_modal_state: ResetModalState::new(),
        }
    }

    pub fn add_modal_state_mut(&mut self) -> &mut AddModalState {
        &mut self.add_modal_state
    }

    pub fn edit_modal_state_mut(&mut self) -> &mut EditModalState {
        &mut self.edit_modal_state
    }

    pub fn progress_modal_state_mut(&mut self) -> &mut ProgressModalState {
        &mut self.progress_modal_state
    }

    pub fn delete_modal_state_mut(&mut self) -> &mut DeleteModalState {
        &mut self.delete_modal_state
    }

    pub fn reset_modal_state_mut(&mut self) -> &mut ResetModalState {
        &mut self.reset_modal_state
    }

    pub fn reset(&mut self) {
        self.add_modal_state.reset();
        self.edit_modal_state.reset();
        self.progress_modal_state.reset();
        self.delete_modal_state.reset();
        self.reset_modal_state.reset();
    }
}

impl Default for ModalState {
    fn default() -> Self {
        Self::new()
    }
}
