use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Padding, Widget},
};

use crate::constants::NOTIFICATION_DISPLAY_TICKS;
use crate::palette::Palette;
use crate::symbols;
use crate::utils::render_ellipsis_if_overflow;

pub struct NotifierMessage {
    pub message: String,
    pub is_error: bool,
}

pub struct Notifier {
    messages: Vec<NotifierMessage>,
    notification_ticks: u16,
    palette: Palette,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            notification_ticks: 0,
            palette: Palette::default(),
        }
    }

    pub fn set_palette(&mut self, palette: Palette) {
        self.palette = palette;
    }

    pub fn notify_success(&mut self, message: &str) {
        self.messages.push(NotifierMessage {
            message: format!("{}  {}", symbols::CHECK_MARK, message),
            is_error: false,
        });
    }

    pub fn notify_error(&mut self, message: &str) {
        self.messages.push(NotifierMessage {
            message: format!("{}  {}", symbols::CROSS_MARK, message),
            is_error: true,
        });
    }

    pub fn tick(&mut self) {
        self.notification_ticks = self.notification_ticks.saturating_add(1);
        if self.notification_ticks >= NOTIFICATION_DISPLAY_TICKS as u16 {
            if !self.messages.is_empty() {
                self.messages.remove(0);
            }
            self.notification_ticks = 0;
        }
    }
}

impl Widget for &Notifier {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.messages.is_empty() {
            return;
        }
        let message = &self.messages[0];
        let p = self.palette;

        let (text_color, border_color, bg_color) = if message.is_error {
            (p.danger, p.danger, p.danger_bg)
        } else {
            (p.accent, p.accent, p.selection)
        };

        let line =
            Line::from(message.message.clone()).style(Style::default().fg(text_color).bg(bg_color));
        let line_width = line.width() as u16 + 4; // +4 for padding (×2) + border (×2)
        let rect_width = line_width.min(80);
        let notifier_area = Rect {
            x: area.right().saturating_sub(rect_width),
            y: area.top().saturating_add(1),
            width: rect_width,
            height: 3,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .padding(Padding::new(1, 1, 0, 0))
            .style(Style::default().bg(bg_color));
        let notifier_inner_area = block.inner(notifier_area);
        block.render(notifier_area, buf);
        line.render(notifier_inner_area, buf);
        render_ellipsis_if_overflow(buf, notifier_inner_area, line_width as usize);
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::new()
    }
}
