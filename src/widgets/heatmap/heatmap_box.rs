use crate::palette::Palette;
use chrono::{Datelike, NaiveDate};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Widget},
};
use std::collections::HashMap;

const WEEKDAYS: [&str; 7] = ["ᴍ", "ᴛ", "ᴡ", "ᴛ", "ꜰ", "ꜱ", "ꜱ"];

pub struct HeatMap<'a> {
    month: &'a str,
    month_num: u32,
    year: i32,
    row_count: &'a u16,
    column_count: &'a u16,
    palette: Palette,
    data: Option<&'a HashMap<String, u8>>,
}

impl<'a> HeatMap<'a> {
    pub(super) fn new(
        month: &'a str,
        month_num: u32,
        year: i32,
        row_count: &'a u16,
        column_count: &'a u16,
        palette: Palette,
        data: Option<&'a HashMap<String, u8>>,
    ) -> Self {
        Self {
            month,
            month_num,
            year,
            row_count,
            column_count,
            palette,
            data,
        }
    }

    fn render_month(&self, area: Rect, buf: &mut Buffer) {
        let p = &self.palette;
        let month_line = Line::from_iter(vec![
            Span::from(self.month.to_string()),
            Span::from(format!(" {}", self.year)),
        ])
        .style(Style::new().fg(p.fg_dim));

        month_line.render(area, buf);

        for (i, weekday) in WEEKDAYS.iter().enumerate() {
            let y = area.y + 1;
            let x = area.x + i as u16 * self.column_count + i as u16;
            if area.contains(Position::new(x, y)) {
                buf.set_string(x, y, weekday, Style::default().fg(p.fg_dim));
            }
        }

        let start_x = area.x;
        let start_y = area.y + 3;

        let row_stride = if *self.row_count == 1 && *self.column_count == 1 {
            1
        } else {
            *self.row_count + 1
        };

        let first = NaiveDate::from_ymd_opt(self.year, self.month_num, 1);
        let first_weekday = first
            .map(|d| d.weekday().num_days_from_monday() as i64)
            .unwrap_or(0);
        let days_in_month = first
            .and_then(|d| {
                let next = if self.month_num == 12 {
                    NaiveDate::from_ymd_opt(self.year + 1, 1, 1)
                } else {
                    NaiveDate::from_ymd_opt(self.year, self.month_num + 1, 1)
                };
                next.map(|n| (n - d).num_days())
            })
            .unwrap_or(30);

        for heatmap_row in 0..6i64 {
            let y = start_y + heatmap_row as u16 * row_stride;

            for weekday in 0..7i64 {
                let x = start_x + weekday as u16 * (*self.column_count + 1);

                let day = heatmap_row * 7 + weekday - first_weekday + 1;

                let value = if day >= 1 && day <= days_in_month {
                    if let Some(data) = self.data {
                        let key = format!("{}-{:02}-{:02}", self.year, self.month_num, day as u32);
                        *data.get(&key).unwrap_or(&0) as usize
                    } else {
                        0
                    }
                } else {
                    0
                };

                let color = p.heatmap[value];

                if *self.row_count == 1 && *self.column_count == 1 {
                    if area.contains(Position::new(x, y)) {
                        buf.set_string(x, y, "■", Style::default().fg(color));
                    }
                } else {
                    let cell_area = Rect::new(x, y, *self.column_count, *self.row_count);
                    if area.intersects(cell_area) {
                        Block::default()
                            .style(Style::default().bg(color))
                            .render(cell_area, buf);
                    }
                }
            }
        }
    }

    fn render_heat_map(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().fg(self.palette.border))
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
