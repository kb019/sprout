use crate::app::App;
use crate::state::modal::ModalState;
use crate::symbols::Symbols;
use crate::widgets::input::Input;
use crate::widgets::simple_list::SimpleList;
use crate::widgets::tile_list::{TileDirection, TileItem, TileList, TileType};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Fill, ListState, Padding, Widget};

pub fn render_modals(app: &mut App, frame: &mut Frame, modal_state: &mut ModalState) {
    let p = app.palette();
    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .reset()
        .fg(p.accent)
        .padding(Padding::new(0, 0, 0, 0))
        .bg(p.background);

    let frame_area = frame.area();
    let modal_area = centered_rect(50, 18, frame_area);
    let modal_inner_area = modal_block.inner(modal_area);

    dim_background(frame.buffer_mut(), frame_area, 0.5);
    clear_modal_area(app, frame, modal_area);

    frame.render_widget(modal_block, modal_area);
    let buf = frame.buffer_mut();
    let left_x = modal_area.left();
    let top_y = modal_area.top();
    let right_x = modal_area.right();
    let bottom_y = modal_area.bottom();
    buf[(left_x, top_y)]
        .set_symbol(" ")
        .set_style(Style::new().fg(p.accent));
    buf[(right_x - 1, top_y)]
        .set_symbol(" ")
        .set_style(Style::new().fg(p.accent));
    buf[(left_x, bottom_y - 1)]
        .set_symbol(" ")
        .set_style(Style::new().fg(p.accent));
    buf[(right_x - 1, bottom_y - 1)]
        .set_symbol(" ")
        .set_style(Style::new().fg(p.accent));

    render_add_modal(app, frame, modal_inner_area, modal_state);
}

fn render_add_modal(app: &mut App, frame: &mut Frame, area: Rect, modal_state: &mut ModalState) {
    let button_state = modal_state.button_state_mut();
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
    render_footer(app, frame, footer_area, button_state);
    render_add_content(app, frame, add_content_inner_area, modal_state);
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
                        "Enter daily goal".to_string(),
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
                        "Enter weekly goal".to_string(),
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
                        "Enter monthly goal".to_string(),
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
                        "Enter yearly goal".to_string(),
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
fn render_footer(app: &mut App, frame: &mut Frame, area: Rect, button_state: &mut ListState) {
    let p = app.palette();

    let border_top = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim))
        .padding(Padding::new(0, 0, 0, 0));
    let footer_inner_area = border_top.inner(area);
    frame.render_widget(border_top, area);
    let selected = button_state.selected().unwrap_or(0);
    let tile_items: Vec<TileItem> = ["Add", "Cancel"]
        .iter()
        .enumerate()
        .map(|(i, &label)| {
            if i == selected {
                TileItem::new(
                    Text::from(label)
                        .add_modifier(Modifier::BOLD)
                        .bg(p.selection)
                        .centered(),
                )
            } else {
                TileItem::new(Text::from(label).centered())
            }
        })
        .collect();
    let tile_list = TileList::new(tile_items)
        .tile_type(TileType::Unbordered)
        .style(Style::new().fg(p.fg_dim))
        .highlight_style(Style::new().fg(p.accent))
        .direction(TileDirection::RightToLeft);
    frame.render_stateful_widget(tile_list, footer_inner_area, button_state);
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn render_title(app: &mut App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let horizontal_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(2)]);
    let [title_content_area, close_button_area] = area.layout(&horizontal_layout);
    let title_text = Text::from(Line::from(" Add habit")).style(Style::new().fg(p.accent));
    let cross_mark =
        Line::from(vec![Span::from(Symbols::CROSS_MARK)]).style(Style::new().fg(p.fg_dim));
    let border_bottom = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Plain)
        .border_style(Style::new().fg(p.fg_dim));
    frame.render_widget(title_text, title_content_area);
    frame.render_widget(cross_mark, close_button_area);
    frame.render_widget(border_bottom, area);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x.saturating_add(area.width.saturating_sub(width) / 2);
    let y = area
        .y
        .saturating_add(area.height.saturating_sub(height) / 2);
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

#[allow(clippy::needless_pass_by_ref_mut)]
fn clear_modal_area(app: &mut App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let fill = Fill::new(" ").style(Style::new().bg(p.background));
    frame.render_widget(fill, area);
}

pub fn dim_background(buf: &mut Buffer, area: Rect, amount: f32) {
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let cell = &mut buf[(x, y)];
            if let Color::Rgb(r, g, b) = cell.fg {
                cell.fg = blend_toward_black(r, g, b, amount);
            }
            if let Color::Rgb(r, g, b) = cell.bg {
                cell.bg = blend_toward_black(r, g, b, amount);
            }
        }
    }
}

fn blend_toward_black(r: u8, g: u8, b: u8, amount: f32) -> Color {
    let f = 1.0 - amount;
    Color::Rgb(
        (r as f32 * f) as u8,
        (g as f32 * f) as u8,
        (b as f32 * f) as u8,
    )
}
