use crate::app::App;
use crate::widgets::heatmap::HeatMapGen;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Padding};

pub fn render_heatmap(_app: &mut App, frame: &mut Frame, area: Rect) {
    let heat_map_block = Block::default()
        .title(" activity heatmap ")
        .borders(Borders::ALL)
        .border_style(Style::new().fg(crate::palette::Palette::BRAND_GREEN))
        .padding(Padding::new(1, 1, 1, 1));
    let block_inner_area = heat_map_block.inner(area);
    frame.render_widget(heat_map_block, area);
    frame.render_widget(HeatMapGen::new(), block_inner_area);
}
