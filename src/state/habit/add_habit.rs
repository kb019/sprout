#[derive(Default)]
pub struct AddHabitState {
    is_adding_habit: bool,
}

impl AddHabitState {
    pub fn new() -> Self {
        Self {
            is_adding_habit: false,
        }
    }

    pub fn is_adding_habit(&self) -> bool {
        self.is_adding_habit
    }

    pub fn set_is_adding_habit(&mut self, is_adding: bool) {
        self.is_adding_habit = is_adding;
    }
}
