use ratatui::widgets::ListState;

use crate::model::habit::Habit;
use crate::state::input::{InputState, InputType};

pub struct EditModalState {
    current_field_focus: usize,
    active_fields: Vec<usize>,
    pub habit_name_input_state: InputState,
    pub daily_goal_input_state: InputState,
    pub weekly_goal_input_state: InputState,
    pub monthly_goal_input_state: InputState,
    pub yearly_goal_input_state: InputState,
    button_state: ListState,
    is_editing: bool,
}

impl EditModalState {
    pub fn new() -> Self {
        let mut state = Self {
            current_field_focus: 0,
            active_fields: vec![0, 1, 2, 3, 4],
            habit_name_input_state: InputState::new(),
            daily_goal_input_state: InputState::new(),
            weekly_goal_input_state: InputState::new(),
            monthly_goal_input_state: InputState::new(),
            yearly_goal_input_state: InputState::new(),
            button_state: ListState::default(),
            is_editing: false,
        };
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
        state.button_state.select(Some(0));
        state.focus_input_field();
        state
    }

    pub fn initialize(&mut self, habit: &Habit) {
        self.reset();
        self.habit_name_input_state.set_value(habit.name.clone());
        if habit.daily_goal > 0 {
            self.daily_goal_input_state
                .set_value(habit.daily_goal.to_string());
        }
        if habit.weekly_goal > 0 {
            self.weekly_goal_input_state
                .set_value(habit.weekly_goal.to_string());
        }
        if habit.monthly_goal > 0 {
            self.monthly_goal_input_state
                .set_value(habit.monthly_goal.to_string());
        }
        if habit.yearly_goal > 0 {
            self.yearly_goal_input_state
                .set_value(habit.yearly_goal.to_string());
        }
        let mut fields = vec![0usize];
        if habit.daily_goal > 0 {
            fields.push(1);
        }
        if habit.weekly_goal > 0 {
            fields.push(2);
        }
        if habit.monthly_goal > 0 {
            fields.push(3);
        }
        if habit.yearly_goal > 0 {
            fields.push(4);
        }
        self.active_fields = fields;
        self.focus_input_field();
    }

    pub fn is_editing(&self) -> bool {
        self.is_editing
    }

    pub fn set_is_editing(&mut self, value: bool) {
        self.is_editing = value;
    }

    pub fn get_current_field_focus(&self) -> usize {
        self.current_field_focus
    }

    pub fn get_current_actual_field(&self) -> usize {
        self.active_fields
            .get(self.current_field_focus)
            .copied()
            .unwrap_or(0)
    }

    pub fn active_fields(&self) -> &[usize] {
        &self.active_fields
    }

    pub fn any_active_goal_field_empty(&self) -> bool {
        self.active_fields
            .iter()
            .filter(|&&f| f > 0)
            .any(|&f| match f {
                1 => self.daily_goal_input_state.get_value().trim().is_empty(),
                2 => self.weekly_goal_input_state.get_value().trim().is_empty(),
                3 => self.monthly_goal_input_state.get_value().trim().is_empty(),
                4 => self.yearly_goal_input_state.get_value().trim().is_empty(),
                _ => false,
            })
    }

    pub fn focus_next_field(&mut self) {
        let max = self.active_fields.len().max(1);
        self.current_field_focus = (self.current_field_focus + 1) % max;
        self.focus_input_field();
    }

    pub fn focus_prev_field(&mut self) {
        let max = self.active_fields.len().max(1);
        if self.current_field_focus == 0 {
            self.current_field_focus = max - 1;
        } else {
            self.current_field_focus -= 1;
        }
        self.focus_input_field();
    }

    fn focus_input_field(&mut self) {
        self.unfocus_all_inputs();
        let actual = self
            .active_fields
            .get(self.current_field_focus)
            .copied()
            .unwrap_or(0);
        match actual {
            0 => self.habit_name_input_state.set_focus(true),
            1 => self.daily_goal_input_state.set_focus(true),
            2 => self.weekly_goal_input_state.set_focus(true),
            3 => self.monthly_goal_input_state.set_focus(true),
            4 => self.yearly_goal_input_state.set_focus(true),
            _ => {}
        }
    }

    fn unfocus_all_inputs(&mut self) {
        self.habit_name_input_state.set_focus(false);
        self.daily_goal_input_state.set_focus(false);
        self.weekly_goal_input_state.set_focus(false);
        self.monthly_goal_input_state.set_focus(false);
        self.yearly_goal_input_state.set_focus(false);
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

impl Default for EditModalState {
    fn default() -> Self {
        Self::new()
    }
}
