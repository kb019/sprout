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

struct NotifierMessage {
    message: String,
    is_error: bool,
    ticks: u16,
}

pub struct Notifier {
    messages: Vec<NotifierMessage>,
    palette: Palette,
    level: usize,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            palette: Palette::default(),
            level: 0,
        }
    }

    pub fn set_palette(&mut self, palette: Palette) {
        self.palette = palette;
    }

    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }

    pub fn notify_success(&mut self, message: &str) {
        if self.level != 0 {
            return;
        }
        self.messages.push(NotifierMessage {
            message: format!("{}  {}", symbols::CHECK_MARK, message),
            is_error: false,
            ticks: 0,
        });
    }

    pub fn notify_error(&mut self, message: &str) {
        if self.level == 2 {
            return;
        }
        self.messages.push(NotifierMessage {
            message: format!("{}  {}", symbols::CROSS_MARK, message),
            is_error: true,
            ticks: 0,
        });
    }

    pub fn tick(&mut self) {
        for msg in &mut self.messages {
            msg.ticks = msg.ticks.saturating_add(1);
        }
        self.messages
            .retain(|msg| msg.ticks < NOTIFICATION_DISPLAY_TICKS as u16);
    }
}

impl Widget for &Notifier {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.messages.is_empty() {
            return;
        }
        let p = self.palette;
        const MAX_WIDTH: u16 = 60;
        const CHROME: u16 = 4; // border (×2) + padding (×2)
        const ITEM_HEIGHT: u16 = 3;
        const GAP: u16 = 1;

        for (i, message) in self.messages.iter().enumerate() {
            let y_offset = area
                .top()
                .saturating_add(1 + i as u16 * (ITEM_HEIGHT + GAP));
            if y_offset + ITEM_HEIGHT > area.bottom() {
                break;
            }

            let (text_color, border_color, bg_color) = if message.is_error {
                (p.danger, p.danger, p.danger_bg)
            } else {
                (p.accent, p.accent, p.selection)
            };

            let line = Line::from(message.message.clone())
                .style(Style::default().fg(text_color).bg(bg_color));
            let line_width = line.width() as u16 + CHROME;
            let rect_width = line_width.min(MAX_WIDTH);

            let notifier_area = Rect {
                x: area.right().saturating_sub(rect_width),
                y: y_offset,
                width: rect_width,
                height: ITEM_HEIGHT,
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .padding(Padding::new(1, 1, 0, 0))
                .style(Style::default().bg(bg_color));
            let inner_area = block.inner(notifier_area);
            block.render(notifier_area, buf);
            render_ellipsis_if_overflow(buf, inner_area, line.width());

            line.render(inner_area, buf);
        }
    }
}

impl Default for Notifier {
    fn default() -> Self {
        Self::new()
    }
}
