use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, ListState, Padding, StatefulWidget, Widget},
};

use crate::{
    app::App,
    palette::Palette,
    widgets::{
        simple_list::SimpleList,
        tile_list::{TileBorderType, TileDirection, TileItem, TileList, TileType},
    },
};

pub fn render_settings_page(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    settings_state: &mut ListState,
    settings_tile_states: &mut [ListState],
) {
    let p = app.palette();
    let settings_block = Block::default()
        .title(" settings ")
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.accent))
        .padding(Padding::new(1, 1, 1, 1));
    let block_inner_area = settings_block.inner(area);
    render_settings(
        app,
        frame,
        block_inner_area,
        settings_state,
        settings_tile_states,
    );
    frame.render_widget(settings_block, area);
}

pub fn render_settings(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    settings_state: &mut ListState,
    settings_tile_states: &mut [ListState],
) {
    let p = app.palette();
    let simple_list =
        SimpleList::new(
            vec!["2", "2", "3"],
            |index, item_area, buf, is_selected| match index {
                0 => render_setting_accent(
                    app,
                    buf,
                    item_area,
                    is_selected,
                    &mut settings_tile_states[0],
                ),
                1 => render_setting_dashboard(
                    app,
                    buf,
                    item_area,
                    is_selected,
                    &mut settings_tile_states[1],
                ),
                2 => render_setting_reset(
                    app,
                    buf,
                    item_area,
                    is_selected,
                    &mut settings_tile_states[2],
                ),
                _ => {}
            },
        )
        .render_line()
        .highlight_background_color(p.row_highlight)
        .line_color(p.fg_dim);
    *settings_state.offset_mut() = 0;
    frame.render_stateful_widget(simple_list, area, settings_state);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_setting_accent(
    app: &mut App,
    buf: &mut Buffer,
    area: Rect,
    is_selected: bool,
    tile_state: &mut ListState,
) {
    let p = app.palette();
    render_setting_row(
        buf,
        area,
        "Accent theme",
        "Sidebar & heatmap color",
        build_items(&app.themes, is_selected, tile_state, p),
        TileType::Unbordered,
        TileBorderType::Rounded,
        Style::new().fg(p.accent),
        tile_state,
        p,
    );
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_setting_dashboard(
    app: &mut App,
    buf: &mut Buffer,
    area: Rect,
    is_selected: bool,
    tile_state: &mut ListState,
) {
    let p = app.palette();
    render_setting_row(
        buf,
        area,
        "Default View on launch",
        "Which tab opens when the app starts",
        build_items(&app.menu, is_selected, tile_state, p),
        TileType::Unbordered,
        TileBorderType::Rounded,
        Style::new().fg(p.accent),
        tile_state,
        p,
    );
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_setting_reset(
    app: &mut App,
    buf: &mut Buffer,
    area: Rect,
    is_selected: bool,
    tile_state: &mut ListState,
) {
    let p = app.palette();
    render_setting_row(
        buf,
        area,
        "Reset all data",
        "Deletes every habit & history — cannot be undone",
        vec![TileItem::new(Text::from("reset").centered().bg(
            if is_selected {
                p.row_highlight
            } else {
                p.background
            },
        ))],
        TileType::Bordered,
        TileBorderType::Sharp,
        Style::new().fg(p.danger),
        tile_state,
        p,
    );
}

fn build_items<S: AsRef<str>>(
    labels: &[S],
    is_selected: bool,
    tile_state: &ListState,
    p: Palette,
) -> Vec<TileItem<'static>> {
    labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let mut modifier = Modifier::empty();
            let mut bg = if is_selected {
                p.row_highlight
            } else {
                p.background
            };
            if tile_state.selected() == Some(i) {
                modifier = Modifier::BOLD;
                bg = p.selection;
            }
            TileItem::new(
                Text::from(label.as_ref().to_string())
                    .add_modifier(modifier)
                    .centered()
                    .bg(bg),
            )
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn render_setting_row(
    buf: &mut Buffer,
    area: Rect,
    title: &str,
    subtitle: &str,
    items: Vec<TileItem<'static>>,
    tile_type: TileType,
    tile_border_type: TileBorderType,
    highlight_style: Style,
    tile_state: &mut ListState,
    p: Palette,
) {
    let [left_area, right_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Min(10)])
        .spacing(1)
        .areas(area);

    Widget::render(
        Text::from_iter([
            Line::from(Span::from(title).fg(p.fg)),
            Line::from(Span::from(subtitle).fg(p.fg_dim)),
        ]),
        left_area,
        buf,
    );

    let tiles = TileList::new(items)
        .style(Style::new().fg(p.fg_dim))
        .highlight_style(highlight_style)
        .tile_type(tile_type)
        .tile_border_type(tile_border_type)
        .direction(TileDirection::RightToLeft);

    StatefulWidget::render(&tiles, right_area, buf, tile_state);
}
