use crate::state::input::{InputState, InputType};

//0 is habit field
//1 is daily goal field
//2 is weekly goal field
//3 is monthly goal field
//4 is yea  rly goal field
pub struct AddModalState {
    current_field_focus: usize,
    habit_name_input_state: InputState,
    daily_goal_input_state: InputState,
    weekly_goal_input_state: InputState,

    monthly_goal_input_state: InputState,
    yearly_goal_input_state: InputState,
}

impl AddModalState {
    pub fn new() -> Self {
        let mut state = Self {
            current_field_focus: 0,
            habit_name_input_state: InputState::new(),
            daily_goal_input_state: InputState::new(),
            weekly_goal_input_state: InputState::new(),
            monthly_goal_input_state: InputState::new(),
            yearly_goal_input_state: InputState::new(),
        };

        state.focus_input_field();
        state
            .daily_goal_input_state
            .set_input_type(InputType::Number);
        state
            .weekly_goal_input_state
            .set_input_type(InputType::Number);
        state
            .monthly_goal_input_state
            .set_input_type(InputType::Number);
        state
            .yearly_goal_input_state
            .set_input_type(InputType::Number);

        state
    }

    pub fn get_current_field_focus(&self) -> usize {
        self.current_field_focus
    }

    pub fn set_current_field_focus(&mut self, focus: usize) {
        self.current_field_focus = focus;
        self.focus_input_field();
    }

    pub fn focus_next_field(&mut self) {
        self.current_field_focus = (self.current_field_focus + 1) % 5;
        self.focus_input_field();
    }

    pub fn focus_prev_field(&mut self) {
        if self.current_field_focus == 0 {
            self.current_field_focus = 4;
        } else {
            self.current_field_focus -= 1;
        }
        self.focus_input_field();
    }

    pub fn focus_input_field(&mut self) {
        self.reset_foculs_for_all_input_fields();
        let current_focus_field = self.current_field_focus;
        if current_focus_field == 0 {
            self.habit_name_input_state.set_focus(true);
        }
        if current_focus_field == 1 {
            self.daily_goal_input_state.set_focus(true);
        }
        if current_focus_field == 2 {
            self.weekly_goal_input_state.set_focus(true);
        }
        if current_focus_field == 3 {
            self.monthly_goal_input_state.set_focus(true);
        }
        if current_focus_field == 4 {
            self.yearly_goal_input_state.set_focus(true);
        }
    }

    fn reset_foculs_for_all_input_fields(&mut self) {
        self.habit_name_input_state.set_focus(false);
        self.daily_goal_input_state.set_focus(false);
        self.weekly_goal_input_state.set_focus(false);
        self.monthly_goal_input_state.set_focus(false);
        self.yearly_goal_input_state.set_focus(false);
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn habit_name_input_state_mut(&mut self) -> &mut InputState {
        &mut self.habit_name_input_state
    }

    pub fn daily_goal_input_state_mut(&mut self) -> &mut InputState {
        &mut self.daily_goal_input_state
    }

    pub fn weekly_goal_input_state_mut(&mut self) -> &mut InputState {
        &mut self.weekly_goal_input_state
    }

    pub fn monthly_goal_input_state_mut(&mut self) -> &mut InputState {
        &mut self.monthly_goal_input_state
    }

    pub fn yearly_goal_input_state_mut(&mut self) -> &mut InputState {
        &mut self.yearly_goal_input_state
    }
}
