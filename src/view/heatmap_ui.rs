use crate::app::App;
use crate::state::States;
use crate::utils::{focus_colors, progress, selection_modifier};
use crate::widgets::heatmap::HeatMapGen;
use crate::widgets::tile_list::{TileItem, TileList, TileType};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, ListState, Padding};

pub fn render_heatmap_page(app: &App, frame: &mut Frame, area: Rect, states: &mut States) {
    let p = app.palette();
    let (border_color, text_color) = focus_colors(app.is_heatmap_in_focus, p);
    let heat_map_block = Block::default()
        .title(" activity heatmap ")
        .title_style(Style::new().fg(text_color))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(border_color))
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
    render_heatmap(app, frame, heatmap_area);
    render_habits(app, frame, habit_tiles_area, states);
    render_legend(app, frame, legend_area);
}

pub fn render_habits(app: &App, frame: &mut Frame, area: Rect, states: &mut States) {
    let p = app.palette();
    let vertical_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(15), Constraint::Max(15)])
        .spacing(2);
    let is_loading = states.active_days_state.is_fetching();
    let active_line = if is_loading {
        Line::from_iter([
            Span::from(format!("{} ", progress(app))).style(Style::new().fg(p.amber)),
            Span::from(" active days").style(Style::new().fg(p.fg_dim)),
        ])
    } else {
        Line::from_iter([
            Span::from(app.active_days.to_string()).style(Style::new().fg(p.accent)),
            Span::from(" active days").style(Style::new().fg(p.fg_dim)),
        ])
    };
    let [habit_tiles_area, active_days_area] = area.layout(&vertical_layout);
    frame.render_widget(active_line, active_days_area);
    render_habit_tiles(
        app,
        frame,
        habit_tiles_area,
        states.app_state.heatmap_tile_state_mut(),
    );
}

pub fn render_heatmap(app: &App, frame: &mut Frame, area: Rect) {
    frame.render_widget(HeatMapGen::new(app.palette()), area);
}

pub fn render_legend(app: &App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let legend_line = Line::from_iter([
        Span::from("less").style(Style::new().fg(p.fg_dim)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(p.heatmap[0])),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(p.heatmap[1])),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(p.heatmap[2])),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(p.heatmap[3])),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(p.heatmap[4])),
        Span::from(" "),
        Span::from("more").style(Style::new().fg(p.fg_dim)),
    ]);
    frame.render_widget(legend_line, area);
}

pub fn render_habit_tiles(app: &App, frame: &mut Frame, area: Rect, tile_state: &mut ListState) {
    let p = app.palette();
    let tiles_items: Vec<TileItem> = app
        .habits
        .iter()
        .enumerate()
        .map(|(i, habit_item)| {
            let bold_modifier = selection_modifier(tile_state, i);
            TileItem::new(
                Text::from(habit_item.name.clone())
                    .add_modifier(bold_modifier)
                    .left_aligned(),
            )
        })
        .collect();

    let tiles = TileList::new(tiles_items)
        .style(Style::new().fg(p.fg_dim))
        .highlight_style(Style::new().fg(p.accent))
        .tile_type(TileType::Bordered);

    frame.render_stateful_widget(tiles, area, tile_state);
}
