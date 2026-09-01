pub enum InputType {
    Text,
    Number,
}
pub struct InputState {
    value: String,
    is_focused: bool,
    input_type: InputType,
    cursor_position: usize,
    cursor_visibility_delay: usize,
    max_length: Option<usize>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            is_focused: false,
            input_type: InputType::Text,
            cursor_position: 0,
            cursor_visibility_delay: 0,
            max_length: None,
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, new_value: String) {
        self.cursor_position = new_value.len();
        self.value = new_value;
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn set_focus(&mut self, focus: bool) {
        self.is_focused = focus;
    }

    pub fn get_input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn set_input_type(&mut self, input_type: InputType) {
        self.input_type = input_type;
    }

    pub fn push_char(&mut self, c: char) {
        if self.max_length.is_some_and(|max| self.value.len() >= max) {
            return;
        }
        self.value.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn set_max_length(&mut self, max: usize) {
        self.max_length = Some(max);
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.value.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.value.len() {
            self.cursor_position += 1;
        }
    }

    pub fn reset(&mut self) {
        self.value.clear();
        self.cursor_position = 0;
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn get_cursor_visibility_delay(&self) -> usize {
        self.cursor_visibility_delay
    }

    pub fn set_cursor_visibility_delay(&mut self, delay: usize) {
        self.cursor_visibility_delay = delay;
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
