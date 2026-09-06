use self::heatmap_box::HeatMap;
use crate::palette::Palette;
use chrono::{Datelike, Local};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Widget,
};
use std::collections::HashMap;

mod heatmap_box;
pub struct HeatMapGen {
    palette: Palette,
    year: i32,
    data: Option<HashMap<String, u8>>,
}

struct HeatMapAreaInfo {
    total_heatmaps_that_can_fit: u16,
    _rows_that_can_fit: u16,
    columns_that_can_fit: u16,
    _rows_required: u16,
    heatmap_height: u16,
    heatmap_width: u16,
}

impl Default for HeatMapGen {
    fn default() -> Self {
        Self::new(Palette::SPROUT)
    }
}

impl HeatMapGen {
    pub fn new(palette: Palette) -> Self {
        Self {
            palette,
            year: Local::now().year(),
            data: None,
        }
    }

    pub fn for_year(mut self, year: i32) -> Self {
        self.year = year;
        self
    }

    pub fn with_data(mut self, data: Option<HashMap<String, u8>>) -> Self {
        self.data = data;
        self
    }

    pub fn cell_dimensions(&self, area: Rect) -> (u16, u16) {
        self.get_suitable_row_column_count(area)
    }

    fn check_if_area_sufficient(
        &self,
        column_count: u16,
        row_count: u16,
        area: Rect,
    ) -> HeatMapAreaInfo {
        let height_required_for_cells = if row_count == 1 && column_count == 1 {
            6 // no inter-row gaps in ■ mode
        } else {
            6 * row_count + 5
        };
        let width_required_for_cells = column_count * 7 + 6; //+6 for gap between cells

        // 5 for top (week, month, border) + 1 for bottom border
        let heatmap_height = height_required_for_cells + 5 + 1;

        // Left + right border
        let heatmap_width = width_required_for_cells + 2 + 2;

        let rows_that_can_fit = (area.height / heatmap_height).max(1); // always try to fit at least one row of heatmaps
        let columns_that_can_fit = (area.width / heatmap_width).max(1); // always try to fit at least one column of heatmaps

        let total_heatmaps_that_can_fit = rows_that_can_fit * columns_that_can_fit;

        let rows_required = if columns_that_can_fit == 0 {
            0
        } else {
            12u16.div_ceil(columns_that_can_fit)
        };

        HeatMapAreaInfo {
            total_heatmaps_that_can_fit,
            _rows_that_can_fit: rows_that_can_fit,
            columns_that_can_fit,
            _rows_required: rows_required,
            heatmap_height,
            heatmap_width,
        }
    }

    //Columns × Rows	Looks
    // 2 × 1
    // 4 × 2
    // 6 × 3

    fn get_suitable_row_column_count(&self, area: Rect) -> (u16, u16) {
        let column_row_count = &[(6, 3), (4, 2), (2, 1), (1, 1)];
        let (column_count, row_count) = column_row_count
            .iter()
            .find(|(c, r)| {
                self.check_if_area_sufficient(*c, *r, area)
                    .total_heatmaps_that_can_fit
                    >= 12
            })
            .unwrap_or(&(1, 1));
        (*column_count, *row_count)
    }

    /// Returns (month_name, month_number 1-12) for each month slot to display.
    fn get_months_to_display(&self, no_of_heat_maps: u16) -> Vec<(&'static str, u32)> {
        const MONTHS: [(&str, u32); 12] = [
            ("Jan", 1),
            ("Feb", 2),
            ("Mar", 3),
            ("Apr", 4),
            ("May", 5),
            ("Jun", 6),
            ("Jul", 7),
            ("Aug", 8),
            ("Sep", 9),
            ("Oct", 10),
            ("Nov", 11),
            ("Dec", 12),
        ];

        if no_of_heat_maps == 0 {
            return vec![];
        }

        let remaining_slots = no_of_heat_maps.saturating_sub(1) as usize;
        let current_month_index = Local::now().month() as usize - 1;
        let left_count = current_month_index.min(remaining_slots);
        let remaining_after_left = remaining_slots.saturating_sub(left_count);
        let right_count = (11 - current_month_index).min(remaining_after_left);
        let index_to_the_left = current_month_index - left_count;
        let index_to_the_right = current_month_index + right_count;

        MONTHS[index_to_the_left..=index_to_the_right].to_vec()
    }

    fn render_heatmap(&self, area: Rect, buf: &mut Buffer) {
        let (column_count, row_count) = self.get_suitable_row_column_count(area);

        let area_info_for_heatmap = self.check_if_area_sufficient(column_count, row_count, area);
        let months_to_display =
            self.get_months_to_display(area_info_for_heatmap.total_heatmaps_that_can_fit);
        let initial_columns = area_info_for_heatmap
            .columns_that_can_fit
            .min(months_to_display.len() as u16);
        let no_of_rows_required = if initial_columns == 0 {
            0
        } else {
            (months_to_display.len() as u16).div_ceil(initial_columns)
        };
        let no_of_columns_per_row = if no_of_rows_required <= 1 {
            area_info_for_heatmap.columns_that_can_fit
        } else {
            (months_to_display.len() as u16).div_ceil(no_of_rows_required)
        };
        let row_flex_layout = if no_of_rows_required > 1 {
            Flex::SpaceBetween
        } else {
            Flex::Start
        };
        let row_layout =
            Layout::vertical(vec![
                Constraint::Length(area_info_for_heatmap.heatmap_height);
                no_of_rows_required as usize
            ])
            .flex(row_flex_layout);
        let row_chunks = row_layout.split(area);

        // Use .enumerate() on the rows loop to get the row index (r_idx)
        for (r_idx, row) in row_chunks
            .iter()
            .take(no_of_rows_required as usize)
            .enumerate()
        {
            let column_flex_layout = if no_of_columns_per_row > 1 {
                Flex::SpaceBetween
            } else {
                Flex::Start
            };
            let column_layout: Layout =
                Layout::horizontal(vec![
                    Constraint::Length(area_info_for_heatmap.heatmap_width);
                    no_of_columns_per_row as usize
                ])
                .flex(column_flex_layout);
            let column_chunks = column_layout.split(*row);

            for (c_idx, column) in column_chunks
                .iter()
                .take(no_of_columns_per_row as usize)
                .enumerate()
            {
                // Calculate the flat index for a 2D grid mapped to a 1D vector
                let flat_index = r_idx * (no_of_columns_per_row as usize) + c_idx;

                if let Some(&(month, month_num)) = months_to_display.get(flat_index) {
                    let heatmap = HeatMap::new(
                        month,
                        month_num,
                        self.year,
                        &row_count,
                        &column_count,
                        self.palette,
                        self.data.as_ref(),
                    );
                    heatmap.render(*column, buf);
                }
            }
        }
    }
}

impl Widget for HeatMapGen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &HeatMapGen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }
        self.render_heatmap(area, buf);
    }
}
