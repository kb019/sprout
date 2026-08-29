pub mod app;
pub mod habit;
pub mod input;
pub mod modal;

use self::app::AppState;
use self::habit::add_habit::AddHabitState;
use self::modal::ModalState;

pub struct States {
    pub app_state: AppState,
    pub modal_state: ModalState,
    pub add_habit_state: AddHabitState,
}

impl States {
    pub fn new() -> Self {
        Self {
            app_state: AppState::new(),
            modal_state: ModalState::new(),
            add_habit_state: AddHabitState::new(),
        }
    }
}

impl Default for States {
    fn default() -> Self {
        Self::new()
    }
}
