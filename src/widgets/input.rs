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
    show_cursor: bool,
}

impl<'a> Input<'a> {
    pub fn new(
        placeholder: String,
        palette: Palette,
        input_state: &'a mut InputState,
        show_cursor: bool,
    ) -> Self {
        Input {
            placeholder,
            palette,
            input_state,
            show_cursor,
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
        let cursor_position = self.input_state.get_cursor_position();
        let visible_width = inner.width as usize;

        // Scroll left: keep cursor at the rightmost visible column when text overflows.
        // Decreasing cursor_position shrinks view_offset, revealing the left portion.
        let view_offset = if showing_placeholder {
            0
        } else {
            cursor_position.saturating_sub(visible_width.saturating_sub(1))
        };

        let raw_text = if showing_placeholder {
            &self.placeholder
        } else {
            self.input_state.get_value()
        };
        let display_text: String = raw_text
            .chars()
            .skip(view_offset)
            .take(visible_width)
            .collect();
        let text_color = if showing_placeholder {
            self.palette.fg_muted
        } else {
            self.palette.fg
        };

        Line::from(Span::styled(
            display_text.as_str(),
            Style::default().fg(text_color),
        ))
        .render(inner, buf);

        if self.input_state.is_focused() && self.show_cursor {
            let cursor_x = inner.left() + (cursor_position - view_offset) as u16;
            if cursor_x < inner.right() {
                buf[(cursor_x, inner.top())].set_style(Style::default().bg(self.palette.accent));
            }
        }
    }
}
