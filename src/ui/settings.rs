use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
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
) {
    let settings_block = Block::default()
        .title(" settings ")
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BRAND_GREEN))
        .padding(Padding::new(1, 1, 1, 1));
    let block_inner_area = settings_block.inner(area);
    render_settings(app, frame, block_inner_area, settings_state);
    frame.render_widget(settings_block, area);
}

pub fn render_settings(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    settings_state: &mut ListState,
) {
    let simple_list =
        SimpleList::new(
            vec!["2", "2", "3"],
            |index, item_area, buf, is_selected| match index {
                0 => render_setting_accent(app, buf, item_area, is_selected),
                1 => render_setting_dashboard(app, buf, item_area, is_selected),
                2 => render_setting_reset(app, buf, item_area, is_selected),
                _ => {}
            },
        )
        .render_line()
        .highlight_background_color(Palette::ROW_HIGHLIGHT)
        .line_color(Palette::TEXT_SECONDARY);
    *settings_state.offset_mut() = 0;
    frame.render_stateful_widget(simple_list, area, settings_state);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_setting_accent(app: &mut App, buf: &mut Buffer, area: Rect, is_selected: bool) {
    render_setting_row(
        buf,
        area,
        "Accent theme",
        "Sidebar & heatmap color",
        build_items(&app.habits, is_selected),
        TileType::Unbordered,
        TileBorderType::Rounded,
        Style::new().fg(Palette::BRAND_GREEN),
    );
}
#[allow(clippy::needless_pass_by_ref_mut)]
fn render_setting_dashboard(app: &mut App, buf: &mut Buffer, area: Rect, is_selected: bool) {
    render_setting_row(
        buf,
        area,
        "Default View on launch",
        "Which tab opens when the app starts",
        build_items(&app.menu, is_selected),
        TileType::Unbordered,
        TileBorderType::Rounded,
        Style::new().fg(Palette::BRAND_GREEN),
    );
}

fn render_setting_reset(_app: &mut App, buf: &mut Buffer, area: Rect, is_selected: bool) {
    render_setting_row(
        buf,
        area,
        "Reset all data",
        "Deletes every habit & history — cannot be undone",
        vec![TileItem::new(Text::from("reset").centered().bg(
            if is_selected {
                Palette::ROW_HIGHLIGHT
            } else {
                Palette::BACKGROUND
            },
        ))],
        TileType::Bordered,
        TileBorderType::Sharp,
        Style::new().fg(Color::Red),
    );
}

fn build_items<S: AsRef<str>>(labels: &[S], is_selected: bool) -> Vec<TileItem<'static>> {
    let mut tile_state = ListState::default();
    tile_state.select_first();
    labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let mut modifier = Modifier::empty();
            let mut bg = if is_selected {
                Palette::ROW_HIGHLIGHT
            } else {
                Palette::BACKGROUND
            };
            if tile_state.selected() == Some(i) {
                modifier = Modifier::BOLD;
                bg = Palette::SELECTION;
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
) {
    let [left_area, right_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Min(10)])
        .spacing(1)
        .areas(area);

    Widget::render(
        Text::from_iter([
            Line::from(Span::from(title).fg(Palette::TEXT_PRIMARY)),
            Line::from(Span::from(subtitle).fg(Palette::TEXT_SECONDARY)),
        ]),
        left_area,
        buf,
    );

    let mut tile_state = ListState::default();
    tile_state.select_first();

    let tiles = TileList::new(items)
        .style(Style::new().fg(Palette::TEXT_SECONDARY))
        .highlight_style(highlight_style)
        .tile_type(tile_type)
        .tile_border_type(tile_border_type)
        .direction(TileDirection::RightToLeft);

    StatefulWidget::render(&tiles, right_area, buf, &mut tile_state);
    *tile_state.offset_mut() = 0;
}
