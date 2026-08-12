use crate::palette::Palette;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Widget},
};

const WEEKDAYS: [&str; 7] = ["ᴍ", "ᴛ", "ᴡ", "ᴛ", "ꜰ", "ꜱ", "ꜱ"];

pub struct HeatMap<'a> {
    month: &'a str,
    row_count: &'a u16,
    column_count: &'a u16,
}

impl<'a> HeatMap<'a> {
    pub(super) fn new(month: &'a str, row_count: &'a u16, column_count: &'a u16) -> Self {
        Self {
            month,
            row_count,
            column_count,
        }
    }

    fn render_month(&self, area: Rect, buf: &mut Buffer) {
        let month_line = Line::from_iter(vec![
            Span::from(self.month.to_string()),
            Span::from(" 2026"),
        ])
        .style(Palette::TEXT_SECONDARY);

        month_line.render(area, buf);

        // Render the weekdays
        for (i, weekday) in WEEKDAYS.iter().enumerate() {
            let y = area.y + 1;
            let x = area.x + i as u16 * self.column_count + i as u16;

            buf.set_string(x, y, weekday, Style::default().fg(Palette::TEXT_SECONDARY));
        }

        // Render the heatmap cells
        let start_x = area.x;
        let start_y = area.y + 3;

        for heatmap_row in 0..6 {
            let y = start_y + heatmap_row * (*self.row_count + 1);

            for weekday in 0..7 {
                let x = start_x + weekday * (*self.column_count + 1);

                // Generate one color for this heatmap cell
                let value = (x.wrapping_mul(31) + y.wrapping_mul(17)) % 5;

                let color = match value {
                    0 => Palette::HEATMAP_0,
                    1 => Palette::HEATMAP_1,
                    2 => Palette::HEATMAP_2,
                    3 => Palette::HEATMAP_3,
                    4 => Palette::HEATMAP_4,
                    _ => unreachable!(),
                };

                let cell_area = Rect::new(x, y, *self.column_count, *self.row_count);

                if area.intersects(cell_area) {
                    let cell = Block::default().style(Style::default().bg(color));

                    cell.render(cell_area, buf);
                }
            }
        }

        // Finished rendering heatmap cells
    }

    fn render_heat_map(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Palette::BORDER))
            .padding(Padding::new(1, 1, 0, 1));

        let block_inner_area = block.inner(area);

        block.render(area, buf);

        self.render_month(block_inner_area, buf);
    }
}

impl Widget for HeatMap<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &HeatMap<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_heat_map(area, buf);
    }
}
