use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    app::App,
    state::States,
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

fn render_title(app: &mut App, frame: &mut Frame, area: Rect, habit_name: &str) {
    let p = app.palette();
    let title = format!(" Delete - {} ", habit_name);
    let title_text = Text::from(Line::from(title)).style(Style::new().fg(p.accent));
    let border_bottom = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim));
    frame.render_widget(title_text, area);
    frame.render_widget(border_bottom, area);
}

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
    let tile_items: Vec<TileItem> = vec![confirm_text]
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            let text = if i == 0 && is_deleting {
                text.add_modifier(Modifier::BOLD).centered()
            } else {
                text.fg(p.danger)
                    .add_modifier(Modifier::BOLD)
                    .bg(p.selection)
                    .centered()
            };
            TileItem::new(text)
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
