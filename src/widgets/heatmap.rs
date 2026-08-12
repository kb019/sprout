use self::heatmap_box::HeatMap;
use chrono::{Datelike, Local};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Widget,
};

mod heatmap_box;
pub struct HeatMapGen {}

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
        Self::new()
    }
}

impl HeatMapGen {
    pub fn new() -> Self {
        Self {}
    }

    fn check_if_area_sufficient(
        &self,
        column_count: u16,
        row_count: u16,
        area: Rect,
    ) -> HeatMapAreaInfo {
        let height_required_for_cells = 6 * row_count + 5; //+5 for gap between cells
        let width_required_for_cells = column_count * 7 + 6 + 1 + 1; //+6 for gap between cells

        // 3 for top (week, month, border) + 1 for bottom border
        let heatmap_height = height_required_for_cells + 3 + 2 + 1;

        // Left + right border
        let heatmap_width = width_required_for_cells + 1 + 1;

        let rows_that_can_fit = area.height / heatmap_height;
        let columns_that_can_fit = area.width / heatmap_width;

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
    // 8 × 4
    // 10 × 5

    fn get_suitable_row_column_count(&self, area: Rect) -> (u16, u16) {
        let column_row_count = &[(14, 7), (12, 6), (10, 5), (8, 4), (6, 3), (4, 2), (2, 1)];
        let (column_count, row_count) = column_row_count
            .iter()
            .find(|(c, r)| {
                self.check_if_area_sufficient(*c, *r, area)
                    .total_heatmaps_that_can_fit
                    >= 12
            })
            .unwrap_or(&(2, 1));
        (*column_count, *row_count)
    }

    /// Get the months to display in the heat map by trying to always fit current month
    fn get_months_to_display(&self, no_of_heat_maps: u16) -> Vec<String> {
        let months_to_display = [
            "Jan".to_string(),
            "Feb".to_string(),
            "Mar".to_string(),
            "Apr".to_string(),
            "May".to_string(),
            "Jun".to_string(),
            "Jul".to_string(),
            "Aug".to_string(),
            "Sep".to_string(),
            "Oct".to_string(),
            "Nov".to_string(),
            "Dec".to_string(),
        ];

        if no_of_heat_maps == 0 {
            return vec![];
        }

        // Reserve one slot for the current month.
        let remaining_slots = no_of_heat_maps.saturating_sub(1) as usize;

        let current_month_index = Local::now().month() as usize - 1;

        // PRIORITIZE MONTHS TO THE LEFT.
        let months_to_the_left = current_month_index;

        let left_count = months_to_the_left.min(remaining_slots);

        let remaining_after_left = remaining_slots.saturating_sub(left_count);

        // Then use whatever space is left for months to the right.
        let months_to_the_right = 11 - current_month_index;
        let right_count = months_to_the_right.min(remaining_after_left);

        let index_to_the_left = current_month_index - left_count;
        let index_to_the_right = current_month_index + right_count;

        months_to_display[index_to_the_left..=index_to_the_right].to_vec()
    }

    fn render_heatmap(&self, area: Rect, buf: &mut Buffer) {
        let (column_count, row_count) = self.get_suitable_row_column_count(area);
        let area_info_for_heatmap = self.check_if_area_sufficient(column_count, row_count, area);
        let months_to_display: Vec<String> =
            self.get_months_to_display(area_info_for_heatmap.total_heatmaps_that_can_fit);
        let no_of_columns_per_row = area_info_for_heatmap
            .columns_that_can_fit
            .min(months_to_display.len() as u16);
        let no_of_rows_required = if no_of_columns_per_row == 0 {
            0
        } else {
            (months_to_display.len() as u16).div_ceil(no_of_columns_per_row)
        };
        let no_of_columns_per_row = area_info_for_heatmap.columns_that_can_fit;
        let row_layout =
            Layout::vertical(vec![
                Constraint::Length(area_info_for_heatmap.heatmap_height);
                no_of_rows_required as usize
            ])
            .flex(Flex::SpaceAround);
        let row_chunks = row_layout.split(area);

        // Use .enumerate() on the rows loop to get the row index (r_idx)
        for (r_idx, row) in row_chunks
            .iter()
            .take(no_of_rows_required as usize)
            .enumerate()
        {
            let column_layout: Layout =
                Layout::horizontal(vec![
                    Constraint::Length(area_info_for_heatmap.heatmap_width);
                    no_of_columns_per_row as usize
                ])
                .flex(Flex::SpaceAround);
            let column_chunks = column_layout.split(*row);

            for (c_idx, column) in column_chunks
                .iter()
                .take(no_of_columns_per_row as usize)
                .enumerate()
            {
                // Calculate the flat index for a 2D grid mapped to a 1D vector
                let flat_index = r_idx * (no_of_columns_per_row as usize) + c_idx;

                if let Some(month) = months_to_display.get(flat_index) {
                    let heatmap = HeatMap::new(month, &row_count, &column_count);
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
