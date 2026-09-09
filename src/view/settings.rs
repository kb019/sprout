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
    utils::{focus_colors, progress, render_ellipsis_if_overflow},
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
    is_saving: bool,
) {
    let p = app.palette();
    let (border_color, text_color) = focus_colors(app.is_settings_in_focus, p);
    let title = if is_saving {
        Line::from(vec![
            Span::from(format!(" {} ", progress(app))).style(Style::new().fg(p.amber)),
            Span::from("settings ").style(Style::new().fg(text_color)),
        ])
    } else {
        Line::from(Span::from(" settings ").style(Style::new().fg(text_color)))
    };
    let settings_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::new().fg(border_color))
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
    let is_settings_in_focus = app.is_settings_in_focus;
    let simple_list = SimpleList::new(
        vec!["2", "2", "2", "3"],
        |index, item_area, buf, is_selected| match index {
            0 => render_setting_accent(
                app,
                buf,
                item_area,
                is_selected,
                &mut settings_tile_states[0],
            ),
            1 => render_setting_cursor_blink(
                app,
                buf,
                item_area,
                is_selected,
                &mut settings_tile_states[1],
            ),
            2 => render_setting_notifications(
                app,
                buf,
                item_area,
                is_selected,
                &mut settings_tile_states[2],
            ),
            3 => render_setting_reset(
                app,
                buf,
                item_area,
                is_selected,
                &mut settings_tile_states[3],
            ),
            _ => {}
        },
    )
    .render_line()
    .highlight_background_color(p.row_highlight)
    .highlight_symbol_color(p.accent)
    .parent_in_focus(is_settings_in_focus)
    .line_color(p.border);
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
fn render_setting_cursor_blink(
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
        "Cursor blink",
        "Enable or Disable cursor blinking",
        build_items(&["On", "Off"], is_selected, tile_state, p),
        TileType::Unbordered,
        TileBorderType::Rounded,
        Style::new().fg(p.accent),
        tile_state,
        p,
    );
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_setting_notifications(
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
        "Notifications",
        "Which notifications are shown",
        build_items(&["All", "Errors", "Off"], is_selected, tile_state, p),
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

fn build_items(
    labels: &[&str],
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
                Text::from(label.to_string())
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

    let [title_area, subtitle_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .areas(left_area);

    let title_line = Line::from(Span::from(title).fg(p.fg));
    let subtitle_line = Line::from(Span::from(subtitle).fg(p.fg_dim));

    Widget::render(&title_line, title_area, buf);
    render_ellipsis_if_overflow(buf, title_area, title_line.width());

    Widget::render(&subtitle_line, subtitle_area, buf);
    render_ellipsis_if_overflow(buf, subtitle_area, subtitle_line.width());

    let tiles = TileList::new(items)
        .style(Style::new().fg(p.fg_dim))
        .highlight_style(highlight_style)
        .tile_type(tile_type)
        .tile_border_type(tile_border_type)
        .direction(TileDirection::RightToLeft);

    StatefulWidget::render(&tiles, right_area, buf, tile_state);
}
