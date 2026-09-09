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

pub fn render_reset_modal(app: &mut App, frame: &mut Frame, area: Rect, states: &mut States) {
    if !app.display_reset_modal {
        return;
    }

    let vertical_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ]);
    let [title_area, content_area, footer_area] = area.layout(&vertical_layout);

    render_title(app, frame, title_area);
    render_content(app, frame, content_area);
    render_footer(app, frame, footer_area, states);
}

fn render_title(app: &mut App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let title_text = Text::from(Line::from(" Reset all data ")).style(Style::new().fg(p.accent));
    let border_bottom = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim));
    frame.render_widget(title_text, area);
    frame.render_widget(border_bottom, area);
}

fn render_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let lines = vec![
        Line::from(vec![
            Span::styled("Reset ", Style::new().fg(p.accent)),
            Span::styled(
                "all habits & history",
                Style::new().fg(p.fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("?", Style::new().fg(p.accent)),
        ]),
        Line::from(Span::styled(
            "This cannot be undone.",
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

fn render_footer(app: &mut App, frame: &mut Frame, area: Rect, states: &mut States) {
    let p = app.palette();
    let is_resetting = states.modal_state.reset_modal_state_mut().is_resetting();

    let border_top = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim));
    let footer_inner_area = border_top.inner(area);
    frame.render_widget(border_top, area);

    let confirm_text = if is_resetting {
        Text::from(Line::from(vec![
            Span::styled(progress(app), Style::new().fg(p.amber)),
            Span::raw(" Confirm"),
        ]))
    } else {
        Text::from("Confirm")
    };

    let reset_modal_state = states.modal_state.reset_modal_state_mut();
    let tile_items: Vec<TileItem> = vec![confirm_text]
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            let text = if i == 0 && is_resetting {
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
        reset_modal_state.button_state_mut(),
    );
}
