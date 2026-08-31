pub mod app;
pub mod habit;
pub mod input;
pub mod modal;

use self::app::AppState;
use self::habit::log_habit::LogHabitState;
use self::modal::ModalState;

pub struct States {
    pub app_state: AppState,
    pub modal_state: ModalState,
    pub log_habit_state: LogHabitState,
}

impl States {
    pub fn new() -> Self {
        Self {
            app_state: AppState::new(),
            modal_state: ModalState::new(),
            log_habit_state: LogHabitState::new(),
        }
    }
}

impl Default for States {
    fn default() -> Self {
        Self::new()
    }
}
