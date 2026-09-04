use crate::app::App;
use crate::palette::Palette;
use crate::state::States;
use crate::utils::{focus_colors, progress, selection_modifier};
use crate::widgets::heatmap::HeatMapGen;
use crate::widgets::simple_list::SimpleList;
use crate::widgets::tile_list::{TileItem, TileList, TileType};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, ListState, Padding, Widget};

pub fn render_heatmap_page(app: &App, frame: &mut Frame, area: Rect, states: &mut States) {
    let p = app.palette();
    let (border_color, text_color) = focus_colors(app.is_heatmap_in_focus, p);
    let title = if states.heatmap_year_habits_state.is_fetching() {
        Line::from(vec![
            Span::from(format!(" {} ", progress(app))).style(Style::new().fg(p.amber)),
            Span::from("activity heatmap ").style(Style::new().fg(text_color)),
        ])
    } else {
        Line::from(Span::from(" activity heatmap ").style(Style::new().fg(text_color)))
    };
    let heat_map_block = Block::default()
        .title(title)
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
    let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Length(6)]).spacing(1);
    let [heatmap_box_area, year_area] = heatmap_area.layout(&horizontal);
    frame.render_widget(heat_map_block, area);
    let heatmap_gen = HeatMapGen::new(p);
    let (col_count, row_count) = heatmap_gen.cell_dimensions(heatmap_box_area);
    frame.render_widget(heatmap_gen, heatmap_box_area);
    render_year_list(app, frame, year_area, p, states);
    render_habits(app, frame, habit_tiles_area, states);
    render_legend(app, frame, legend_area, col_count, row_count);
}

/// Returns the habit IDs visible for the currently selected year, or all if nothing selected.
pub fn selected_year_habit_ids(app: &App, states: &States) -> Vec<i32> {
    let idx = states.app_state.year_list_state().selected().unwrap_or(0);
    app.heatmap_year_habits
        .get(idx)
        .map(|(_, ids)| ids.clone())
        .unwrap_or_default()
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
    let year_habit_ids = selected_year_habit_ids(app, states);
    let year_idx = states.app_state.year_list_state().selected().unwrap_or(0);
    let year = app
        .heatmap_year_habits
        .get(year_idx)
        .map(|(y, _)| *y)
        .unwrap_or(0);
    if let Some(tile_state) = states.app_state.heatmap_tile_state_for_year_mut(year) {
        render_habit_tiles(app, frame, habit_tiles_area, tile_state, &year_habit_ids);
    }
}

pub fn render_legend(app: &App, frame: &mut Frame, area: Rect, col_count: u16, row_count: u16) {
    let p = app.palette();
    let use_square: bool = row_count == 1 && col_count == 1;
    let cell_str = " ".repeat(col_count as usize);

    let mut spans = vec![
        Span::from("less").style(Style::new().fg(p.fg_dim)),
        Span::from(" "),
    ];
    for color in p.heatmap {
        if use_square {
            spans.push(Span::from("■").style(Style::new().fg(color)));
        } else {
            spans.push(Span::from(cell_str.clone()).style(Style::new().bg(color)));
        }
        spans.push(Span::from(" "));
    }
    spans.push(Span::from("more").style(Style::new().fg(p.fg_dim)));

    frame.render_widget(Line::from(spans), area);
}

fn render_year_list(app: &App, frame: &mut Frame, area: Rect, p: Palette, states: &mut States) {
    let years: Vec<i32> = app.heatmap_year_habits.iter().map(|(y, _)| *y).collect();
    let len = years.len();

    let list = SimpleList::new(vec!["1"; len], move |index, item_area, buf, _| {
        if let Some(&year) = years.get(index) {
            Widget::render(
                Line::from(year.to_string())
                    .style(Style::new().fg(p.fg_dim))
                    .centered(),
                item_area,
                buf,
            );
        }
    })
    .highlight_background_color(p.row_highlight)
    .render_line()
    .line_color(p.border);

    let list_state = states.app_state.year_list_state_mut();
    frame.render_stateful_widget(list, area, list_state);
}

pub fn render_habit_tiles(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    tile_state: &mut ListState,
    year_habit_ids: &[i32],
) {
    let p = app.palette();
    let habits_for_year: Vec<_> = app
        .habits
        .iter()
        .filter(|h: &&crate::model::habit::Habit| year_habit_ids.contains(&h.id))
        .collect();
    let tiles_items: Vec<TileItem> = habits_for_year
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
