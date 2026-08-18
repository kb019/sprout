use crate::app::App;
use crate::palette::Palette;
use crate::widgets::heatmap::HeatMapGen;
use crate::widgets::tile_list::{TileItem, TileList, TileType};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, ListState, Padding};

pub fn render_heatmap_page(app: &App, frame: &mut Frame, area: Rect, tile_state: &mut ListState) {
    let heat_map_block = Block::default()
        .title(" activity heatmap ")
        .borders(Borders::ALL)
        .border_style(Style::new().fg(crate::palette::Palette::BRAND_GREEN))
        .padding(Padding::new(1, 1, 1, 1));
    let block_inner_area = heat_map_block.inner(area);
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
    let [habit_tiles_area, heatmap_area, legend_area] = block_inner_area.layout(&vertical_layout);
    frame.render_widget(heat_map_block, area);
    render_heatmap(frame, heatmap_area);
    render_habits(app, frame, habit_tiles_area, tile_state);
    render_lengend(frame, legend_area);
}

pub fn render_habits(app: &App, frame: &mut Frame, area: Rect, tile_state: &mut ListState) {
    let vertical_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(15), Constraint::Max(15)])
        .spacing(2);
    let active_line = Line::from_iter([
        Span::from("176").style(Style::new().fg(Palette::BRAND_GREEN)),
        Span::from(" active days").style(Style::new().fg(Palette::TEXT_SECONDARY)),
    ]);
    let [habit_tiles_area, active_days_area] = area.layout(&vertical_layout);
    frame.render_widget(active_line, active_days_area);
    render_habit_tiles(app, frame, habit_tiles_area, tile_state);
}

pub fn render_heatmap(frame: &mut Frame, area: Rect) {
    frame.render_widget(HeatMapGen::new(), area);
}

pub fn render_lengend(frame: &mut Frame, area: Rect) {
    let legend_line = Line::from_iter([
        Span::from("less").style(Style::new().fg(Palette::TEXT_SECONDARY)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_0)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_1)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_2)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_3)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_4)),
        Span::from(" "),
        Span::from("more").style(Style::new().fg(Palette::TEXT_SECONDARY)),
    ]);
    frame.render_widget(legend_line, area);
}

pub fn render_habit_tiles(app: &App, frame: &mut Frame, area: Rect, tile_state: &mut ListState) {
    let tiles_items: Vec<TileItem> = app
        .habits
        .iter()
        .enumerate()
        .map(|(i, menu_item)| {
            let mut bold_modifier = Modifier::empty();
            if let Some(select) = tile_state.selected()
                && select == i
            {
                bold_modifier = Modifier::BOLD;
            }
            TileItem::new(
                Text::from(menu_item.clone())
                    .add_modifier(bold_modifier)
                    .centered(),
            )
        })
        .collect();

    let tiles = TileList::new(tiles_items)
        .style(Style::new().fg(Palette::TEXT_SECONDARY))
        .highlight_style(Style::new().fg(Palette::BRAND_GREEN))
        .tile_type(TileType::Bordered);

    frame.render_stateful_widget(tiles, area, tile_state);

    // Reset offset so items reflow correctly on resize
    *tile_state.offset_mut() = 0;
}
