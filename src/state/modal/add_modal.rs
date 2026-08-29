use ratatui::widgets::ListState;

use crate::state::input::{InputState, InputType};

// current_field_focus:
// 0 = habit name input
// 1 = daily goal input
// 2 = weekly goal input
// 3 = monthly goal input
// 4 = yearly goal input
pub struct AddModalState {
    current_field_focus: usize,
    habit_name_input_state: InputState,
    daily_goal_input_state: InputState,
    weekly_goal_input_state: InputState,
    monthly_goal_input_state: InputState,
    yearly_goal_input_state: InputState,
    button_state: ListState, // 0 = Add, 1 = Cancel
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
            button_state: ListState::default(),
        };

        state.focus_input_field();
        state
            .daily_goal_input_state
            .set_input_type(InputType::Number);
        state.daily_goal_input_state.set_max_length(10);
        state
            .weekly_goal_input_state
            .set_input_type(InputType::Number);
        state.weekly_goal_input_state.set_max_length(10);
        state
            .monthly_goal_input_state
            .set_input_type(InputType::Number);
        state.monthly_goal_input_state.set_max_length(10);
        state
            .yearly_goal_input_state
            .set_input_type(InputType::Number);
        state.yearly_goal_input_state.set_max_length(10);
        state.button_state.select(Some(0)); // Add button selected initially
        state
    }

    // --- field focus ---

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
        self.unfocus_all_inputs();
        match self.current_field_focus {
            0 => self.habit_name_input_state.set_focus(true),
            1 => self.daily_goal_input_state.set_focus(true),
            2 => self.weekly_goal_input_state.set_focus(true),
            3 => self.monthly_goal_input_state.set_focus(true),
            4 => self.yearly_goal_input_state.set_focus(true),
            _ => {}
        }
    }

    pub fn unfocus_all_inputs(&mut self) {
        self.habit_name_input_state.set_focus(false);
        self.daily_goal_input_state.set_focus(false);
        self.weekly_goal_input_state.set_focus(false);
        self.monthly_goal_input_state.set_focus(false);
        self.yearly_goal_input_state.set_focus(false);
    }

    // --- button navigation (TileList: 0 = Add, 1 = Cancel) ---

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

    // --- reset ---

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    // --- input state accessors ---

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
