use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Widget;

use crate::vendor::barchart::Bar;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BarGroup<'a> {
    pub(super) label: Option<Line<'a>>,
    pub(super) bars: Vec<Bar<'a>>,
}

impl<'a> BarGroup<'a> {
    pub fn new<T: Into<Vec<Bar<'a>>>>(bars: T) -> Self {
        Self {
            bars: bars.into(),
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::text::Line;

    use super::*;

    #[test]
    fn test_bargroup_new() {
        let group = BarGroup::new([Bar::with_label("Label1", 1), Bar::with_label("Label2", 2)]);
        assert_eq!(group.bars.len(), 2);
        assert_eq!(group.bars[0].value, 1);
        assert_eq!(group.bars[1].label, Some(Line::from("Label2")));
    }
}

impl<'a> BarGroup<'a> {
    pub(super) fn max(&self) -> Option<u64> {
        self.bars.iter().max_by_key(|v| v.value).map(|v| v.value)
    }

    pub(super) fn render_label(&self, buf: &mut Buffer, area: Rect, default_label_style: Style) {
        if let Some(label) = &self.label {
            let width = label.width() as u16;
            let area = match label.alignment {
                Some(Alignment::Center) => Rect {
                    x: area.x + (area.width.saturating_sub(width)) / 2,
                    width,
                    ..area
                },
                Some(Alignment::Right) => Rect {
                    x: area.x + area.width.saturating_sub(width),
                    width,
                    ..area
                },
                _ => Rect { width, ..area },
            };
            buf.set_style(area, default_label_style);
            label.render(area, buf);
        }
    }
}
