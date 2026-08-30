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

pub fn render_add_modal(app: &mut App, frame: &mut Frame, area: Rect, states: &mut States) {
    let vertical_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(2),
    ]);
    let [title_area, add_content_area, footer_area] = area.layout(&vertical_layout);
    let content_block = Block::default()
        .bg(Color::Rgb(13, 20, 15))
        .padding(Padding::new(1, 1, 1, 1));
    let add_content_inner_area = content_block.inner(add_content_area);

    frame.render_widget(content_block, add_content_area);
    render_title(app, frame, title_area);
    render_add_content(app, frame, add_content_inner_area, &mut states.modal_state);
    render_footer(app, frame, footer_area, states);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_add_content(app: &mut App, frame: &mut Frame, area: Rect, modal_state: &mut ModalState) {
    let add_modal_state = modal_state.add_modal_state_mut();
    let p = app.palette();
    let focused = add_modal_state.get_current_field_focus();
    let show_cursor = !app.cursor_blink_enabled || app.tick_count == 0;
    let list = SimpleList::new(
        vec!["3", "3", "3", "3", "3"],
        move |index, item_rect, buf, _| {
            let input = match index {
                0 => {
                    let habit_input_state = add_modal_state.habit_name_input_state_mut();
                    let habit_input_cursor_delay = habit_input_state.get_cursor_visibility_delay();
                    habit_input_state
                        .set_cursor_visibility_delay(habit_input_cursor_delay.saturating_sub(1));
                    Input::new(
                        "Enter habit name".to_string(),
                        p,
                        habit_input_state,
                        show_cursor,
                    )
                }
                1 => {
                    let daily_goal_input_state = add_modal_state.daily_goal_input_state_mut();
                    let daily_goal_input_cursor_delay =
                        daily_goal_input_state.get_cursor_visibility_delay();
                    daily_goal_input_state.set_cursor_visibility_delay(
                        daily_goal_input_cursor_delay.saturating_sub(1),
                    );
                    Input::new(
                        "Enter daily goal (Optional)".to_string(),
                        p,
                        daily_goal_input_state,
                        show_cursor,
                    )
                }
                2 => {
                    let weekly_goal_input_state = add_modal_state.weekly_goal_input_state_mut();
                    let weekly_goal_input_cursor_delay =
                        weekly_goal_input_state.get_cursor_visibility_delay();
                    weekly_goal_input_state.set_cursor_visibility_delay(
                        weekly_goal_input_cursor_delay.saturating_sub(1),
                    );
                    Input::new(
                        "Enter weekly goal (Optional)".to_string(),
                        p,
                        weekly_goal_input_state,
                        show_cursor,
                    )
                }
                3 => {
                    let monthly_goal_input_state = add_modal_state.monthly_goal_input_state_mut();
                    let monthly_goal_input_cursor_delay =
                        monthly_goal_input_state.get_cursor_visibility_delay();
                    monthly_goal_input_state.set_cursor_visibility_delay(
                        monthly_goal_input_cursor_delay.saturating_sub(1),
                    );
                    Input::new(
                        "Enter monthly goal (Optional)".to_string(),
                        p,
                        monthly_goal_input_state,
                        show_cursor,
                    )
                }
                _ => {
                    let yearly_goal_input_state = add_modal_state.yearly_goal_input_state_mut();
                    let yearly_goal_input_cursor_delay =
                        yearly_goal_input_state.get_cursor_visibility_delay();
                    yearly_goal_input_state.set_cursor_visibility_delay(
                        yearly_goal_input_cursor_delay.saturating_sub(1),
                    );
                    Input::new(
                        "Enter yearly goal (Optional)".to_string(),
                        p,
                        yearly_goal_input_state,
                        show_cursor,
                    )
                }
            };
            Widget::render(&input, item_rect, buf);
        },
    );

    let mut list_state = ListState::default().with_selected(Some(focused));
    frame.render_stateful_widget(list, area, &mut list_state);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_footer(app: &mut App, frame: &mut Frame, area: Rect, states: &mut States) {
    let p = app.palette();

    let border_top = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim))
        .padding(Padding::new(0, 0, 0, 0));
    let footer_inner_area = border_top.inner(area);
    frame.render_widget(border_top, area);

    let is_adding = states.add_habit_state.is_adding_habit();

    let add_text = if is_adding {
        Text::from(Line::from(vec![
            Span::styled(progress(app), Style::new().fg(p.amber)),
            Span::raw(" Add"),
        ]))
    } else {
        Text::from("Add")
    };

    let add_modal_state = states.modal_state.add_modal_state_mut();
    let selected = add_modal_state.selected_button();
    let tile_items: Vec<TileItem> = vec![add_text, Text::from("Cancel")]
        .into_iter()
        .enumerate()
        .map(|(i, text)| {
            if i == selected {
                let text = if i == 0 && is_adding {
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
        add_modal_state.button_state_mut(),
    );
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_title(app: &mut App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let horizontal_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(2)]);
    let [title_content_area, close_button_area] = area.layout(&horizontal_layout);
    let title_text = Text::from(Line::from(" Add habit")).style(Style::new().fg(p.accent));
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
