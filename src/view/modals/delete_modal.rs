use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::App,
    state::States,
    symbols,
    utils::progress,
    widgets::tile_list::{TileDirection, TileItem, TileList, TileType},
};

pub fn render_delete_modal(app: &mut App, frame: &mut Frame, area: Rect, states: &mut States) {
    let Some(habit_id) = app.display_delete_modal else {
        return;
    };

    let habit_name = app
        .habits
        .iter()
        .find(|h| h.id == habit_id)
        .map(|h| h.name.clone())
        .unwrap_or_else(|| "Habit".to_string());

    let vertical_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ]);
    let [title_area, content_area, footer_area] = area.layout(&vertical_layout);

    render_title(app, frame, title_area, &habit_name);
    render_content(app, frame, content_area, &habit_name);
    render_footer(app, frame, footer_area, states, habit_id);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_title(app: &mut App, frame: &mut Frame, area: Rect, habit_name: &str) {
    let p = app.palette();
    let horizontal_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(2)]);
    let [title_content_area, close_button_area] = area.layout(&horizontal_layout);

    let title = format!(" Delete - {} ", habit_name);
    let title_text = Text::from(Line::from(title)).style(Style::new().fg(p.accent));
    let cross_mark =
        Line::from(vec![Span::from(symbols::CROSS_MARK)]).style(Style::new().fg(p.fg_dim));
    let border_bottom = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim));

    frame.render_widget(title_text, title_content_area);
    frame.render_widget(cross_mark, close_button_area);
    frame.render_widget(border_bottom, area);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_content(app: &mut App, frame: &mut Frame, area: Rect, habit_name: &str) {
    let p = app.palette();
    let lines = vec![
        Line::from(vec![
            Span::styled("Delete ", Style::new().fg(p.accent)),
            Span::styled(
                format!("\"{}\"", habit_name),
                Style::new().fg(p.fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("?", Style::new().fg(p.accent)),
        ]),
        Line::from(Span::styled(
            "This removes its history too.",
            Style::new().fg(p.fg_dim),
        )),
    ];
    let [_, center_area, _] = area.layout(&Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(2),
        Constraint::Fill(1),
    ]));
    let paragraph = Paragraph::new(Text::from(lines)).centered();
    frame.render_widget(paragraph, center_area);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_footer(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    states: &mut States,
    _habit_id: i32,
) {
    let p = app.palette();
    let is_deleting = states.modal_state.delete_modal_state_mut().is_deleting();

    let border_top = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim));
    let footer_inner_area = border_top.inner(area);
    frame.render_widget(border_top, area);

    let confirm_text = if is_deleting {
        Text::from(Line::from(vec![
            Span::styled(progress(app), Style::new().fg(p.amber)),
            Span::raw(" Confirm"),
        ]))
    } else {
        Text::from("Confirm")
    };

    let delete_modal_state = states.modal_state.delete_modal_state_mut();
    let selected = delete_modal_state.selected_button();
    let tile_items: Vec<TileItem> = vec![confirm_text, Text::from("Cancel")]
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            if i == selected {
                let text = if i == 0 && is_deleting {
                    text.add_modifier(Modifier::BOLD).centered()
                } else {
                    text.fg(if i == 0 { Color::Red } else { p.fg_dim })
                        .add_modifier(Modifier::BOLD)
                        .bg(if i == 1 { p.selection } else { p.background })
                        .centered()
                };
                TileItem::new(text)
            } else {
                TileItem::new(text.centered())
            }
        })
        .collect();

    let tile_list = TileList::new(tile_items)
        .tile_type(TileType::Unbordered)
        .style(Style::new().fg(p.fg_dim))
        .highlight_style(Style::new())
        .direction(TileDirection::RightToLeft);
    frame.render_stateful_widget(
        tile_list,
        footer_inner_area,
        delete_modal_state.button_state_mut(),
    );
}
