use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, ListState, Padding, Widget},
};

use crate::{
    app::App,
    state::{States, modal::ModalState},
    symbols,
    utils::progress,
    widgets::{
        input::Input,
        simple_list::SimpleList,
        tile_list::{TileDirection, TileItem, TileList, TileType},
    },
};

pub fn render_progress_modal(app: &mut App, frame: &mut Frame, area: Rect, states: &mut States) {
    let Some(habit_id) = app.progress_modal_for_habit_id else {
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

    let content_block = Block::default()
        .bg(Color::Rgb(13, 20, 15))
        .padding(Padding::new(1, 1, 1, 1));
    let content_inner_area = content_block.inner(content_area);
    frame.render_widget(content_block, content_area);

    render_title(app, frame, title_area, &habit_name);
    render_progress_content(app, frame, content_inner_area, &mut states.modal_state);
    render_footer(app, frame, footer_area, states, habit_id);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_title(app: &mut App, frame: &mut Frame, area: Rect, habit_name: &str) {
    let p = app.palette();
    let horizontal_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(2)]);
    let [title_content_area, close_button_area] = area.layout(&horizontal_layout);

    let title = format!(" Log - {} ", habit_name);
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
fn render_progress_content(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    modal_state: &mut ModalState,
) {
    let progress_modal_state = modal_state.progress_modal_state_mut();
    let p = app.palette();
    let show_cursor = !app.cursor_blink_enabled || app.tick_count == 0;

    let list = SimpleList::new(vec!["3"], move |_index, item_rect, buf, _| {
        let progress_input_state = progress_modal_state.progress_input_state_mut();
        let cursor_delay = progress_input_state.get_cursor_visibility_delay();
        progress_input_state.set_cursor_visibility_delay(cursor_delay.saturating_sub(1));
        let input = Input::new(
            "Enter progress value".to_string(),
            p,
            progress_input_state,
            show_cursor,
        );
        Widget::render(&input, item_rect, buf);
    })
    .disable_highlight_symbol()
    .parent_in_focus(true);

    let mut list_state = ListState::default().with_selected(Some(0));
    frame.render_stateful_widget(list, area, &mut list_state);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_footer(app: &mut App, frame: &mut Frame, area: Rect, states: &mut States, habit_id: i32) {
    let p = app.palette();
    let is_logging = states.log_habit_state.is_habit_logging(habit_id);

    let border_top = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim));
    let footer_inner_area = border_top.inner(area);
    frame.render_widget(border_top, area);

    let log_text = if is_logging {
        Text::from(Line::from(vec![
            Span::styled(progress(app), Style::new().fg(p.amber)),
            Span::raw(" Log"),
        ]))
    } else {
        Text::from("Log")
    };

    let progress_modal_state = states.modal_state.progress_modal_state_mut();
    let selected = progress_modal_state.selected_button();
    let tile_items: Vec<TileItem> = vec![log_text, Text::from("Cancel")]
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            if i == selected {
                let text = if i == 0 && is_logging {
                    text.add_modifier(Modifier::BOLD).bg(p.selection).centered()
                } else {
                    text.fg(p.accent)
                        .add_modifier(Modifier::BOLD)
                        .bg(p.selection)
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
        progress_modal_state.button_state_mut(),
    );
}
