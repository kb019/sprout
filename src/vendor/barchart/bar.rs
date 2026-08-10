use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::{Style, Styled};
use ratatui::text::Line;
use ratatui::widgets::Widget;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Bar<'a> {
    pub(super) value: u64,
    pub(super) label: Option<Line<'a>>,
    pub(super) style: Style,
    pub(super) value_style: Style,
    pub(super) text_value: Option<String>,
}

impl<'a> Bar<'a> {
    pub const fn new(value: u64) -> Self {
        Self {
            value,
            label: None,
            style: Style::new(),
            value_style: Style::new(),
            text_value: None,
        }
    }

    pub fn with_label<T: Into<Line<'a>>>(label: T, value: u64) -> Self {
        Self {
            value,
            label: Some(label.into()),
            style: Style::new(),
            value_style: Style::new(),
            text_value: None,
        }
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn value(mut self, value: u64) -> Self {
        self.value = value;
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label<T: Into<Line<'a>>>(mut self, label: T) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn value_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.value_style = style.into();
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn text_value<T: Into<String>>(mut self, text_value: T) -> Self {
        self.text_value = Some(text_value.into());
        self
    }

    pub(super) fn render_value(
        &self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_value_style: Style,
        ticks: u64,
    ) {
        if self.value != 0 {
            const TICKS_PER_LINE: u64 = 8;
            let value = self.value.to_string();
            let value_label = self.text_value.as_ref().unwrap_or(&value);
            let width = value_label.width() as u16;
            if width < max_width || (width == max_width && ticks >= TICKS_PER_LINE) {
                let x = x + (max_width.saturating_sub(value_label.len() as u16) >> 1);
                let y = y.saturating_sub((ticks / TICKS_PER_LINE + 1) as u16);
                if buf.area().contains(Position::new(x, y)) {
                    buf.set_string(
                        x,
                        y,
                        value_label,
                        default_value_style.patch(self.value_style),
                    );
                }
            }
        }
    }

    pub(super) fn render_label(
        &self,
        buf: &mut Buffer,
        max_width: u16,
        x: u16,
        y: u16,
        default_label_style: Style,
    ) {
        let width = self
            .label
            .as_ref()
            .map_or(0, Line::width)
            .min(max_width as usize) as u16;
        let area = Rect {
            x: x + (max_width.saturating_sub(width)) / 2,
            y,
            width,
            height: 1,
        };
        buf.set_style(area, default_label_style);
        if let Some(label) = &self.label {
            label.render(area, buf);
        }
    }
}

impl Styled for Bar<'_> {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(mut self, style: S) -> Self::Item {
        self.style = style.into();
        self
    }
}

//TODO : Add tests for the top value rendering behaviour, i.e the value should appear at top instead of bottom

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Modifier, Style, Stylize};
    use ratatui::text::Line;

    use super::*;

    #[test]
    fn test_bar_new() {
        let bar = Bar::new(42).label(Line::from("Label"));
        assert_eq!(bar.label, Some(Line::from("Label")));
        assert_eq!(bar.value, 42);
    }

    #[test]
    fn test_bar_with_label() {
        let bar = Bar::with_label("Label", 42);
        assert_eq!(bar.label, Some(Line::from("Label")));
        assert_eq!(bar.value, 42);
    }

    #[test]
    fn test_bar_stylized() {
        let bar = Bar::default().red().bold();
        assert_eq!(
            bar.style,
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        );
    }
}
