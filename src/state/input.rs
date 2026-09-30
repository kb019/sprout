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
        if self
            .max_length
            .is_some_and(|max| new_value.chars().count() >= max)
        {
            return;
        }
        self.cursor_position = new_value.chars().count();
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
        if self
            .max_length
            .is_some_and(|max| self.value.chars().count() >= max)
        {
            return;
        }
        let byte_idx = self.byte_index(self.cursor_position);
        self.value.insert(byte_idx, c);
        self.cursor_position += 1;
    }

    pub fn set_max_length(&mut self, max: usize) {
        self.max_length = Some(max);
    }

    pub fn backspace(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        let start = self.byte_index(self.cursor_position - 1);
        let end = self.byte_index(self.cursor_position);
        self.value.replace_range(start..end, "");
        self.cursor_position -= 1;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.value.chars().count() {
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

    fn byte_index(&self, char_idx: usize) -> usize {
        self.value
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.value.len())
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typing_ascii_after_multibyte_advances_cursor_by_one_char() {
        let mut s = InputState::new();
        s.push_char('あ');
        assert_eq!(s.get_cursor_position(), 1);
        s.push_char('a');
        assert_eq!(s.get_value(), "あa");
        assert_eq!(s.get_cursor_position(), 2);
    }
}
