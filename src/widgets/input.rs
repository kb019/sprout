use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

use crate::{palette::Palette, state::input::InputState};

pub enum InputType {
    Text,
    Number,
}

pub struct Input<'a> {
    placeholder: String,
    palette: Palette,
    input_state: &'a mut InputState,
}

impl<'a> Input<'a> {
    pub fn new(placeholder: String, palette: Palette, input_state: &'a mut InputState) -> Self {
        Input {
            placeholder,
            palette,
            input_state,
        }
    }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // area should be at least 3 rows tall: top border, content, bottom border
        let border_color = if self.input_state.is_focused() {
            self.palette.accent
        } else {
            self.palette.border
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(self.palette.background)); // explicit dark fill — this is the actual fix
        let block_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 3, // fixed height for the input box
        };

        let inner = block.inner(block_area);
        block.render(block_area, buf); // Block writes its own cells correctly — no manual buf.get_mut() needed

        let showing_placeholder = self.input_state.get_value().is_empty();
        let text: &str = if showing_placeholder {
            &self.placeholder
        } else {
            self.input_state.get_value()
        };
        let text_color = if showing_placeholder {
            self.palette.fg_muted
        } else {
            self.palette.fg
        };

        let spans = vec![Span::styled(text, Style::default().fg(text_color))];
        let cursor_position = self.input_state.get_cursor_position();

        Line::from(spans).render(inner, buf);
        if self.input_state.is_focused() {
            buf[(area.left() + cursor_position as u16 + 1, area.top() + 1)]
                .set_style(Style::default().fg(self.palette.accent))
                .set_symbol("█");
        }
    }
}
