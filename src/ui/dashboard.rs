use crate::app::App;
use crate::palette::Palette;
use crate::state::State;
use crate::symbols::Symbols;
use crate::utils::{focus_colors, render_ellipsis_if_overflow};
use crate::widgets::simple_list::SimpleList;
use crate::widgets::tile_list::{TileItem, TileList, TileType};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::symbols::line::HORIZONTAL;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, ListState, Padding, Paragraph, Widget};

pub fn render_dashboard(app: &mut App, frame: &mut Frame, app_area: Rect, app_state: &mut State) {
    let p = app.palette();
    let horizontal =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).spacing(1);
    let [dashboard_column, stats_column] = app_area.layout(&horizontal);
    let (border_color, text_color) = focus_colors(app.is_dashboard_in_focus, p);
    let dashboard_block = Block::default()
        .title(" active habits ")
        .title_style(Style::new().fg(text_color))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(border_color))
        .padding(Padding::new(1, 1, 1, 1));
    let dashboard_inner_area = dashboard_block.inner(dashboard_column);
    frame.render_widget(dashboard_block, dashboard_column);
    // render_empty_state(frame, dashboard_inner_area, " No active habits ", p);
    render_habits_content(
        app,
        frame,
        dashboard_inner_area,
        app_state.dashboard_habits_state_mut(),
    );
    let (goal_tile_state, goal_row_states) = app_state.goal_progress_states_mut();
    let tab = goal_tile_state.selected().unwrap_or(0);
    let goal_row_state = &mut goal_row_states[tab];
    render_stats_column(app, frame, stats_column, goal_tile_state, goal_row_state);
}

pub fn render_habits_content(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    habits_state: &mut ListState,
) {
    let p = app.palette();
    let vertical_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [header_area, habits_list_area] = area.layout(&vertical_layout);
    render_habits_header(frame, header_area, p);
    render_habits_list(app, frame, habits_list_area, habits_state);
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn render_habits_list(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    habits_state: &mut ListState,
) {
    let p = app.palette();
    let simple_list =
        SimpleList::new(
            vec!["2", "2", "2"],
            |index, item_area, buf, is_selected| match index {
                0 => render_habit_item(item_area, buf, is_selected, "Morning Run", true, p),
                1 => render_habit_item(item_area, buf, is_selected, "No Sugar", false, p),
                2 => render_habit_item(item_area, buf, is_selected, "Read", true, p),
                _ => {}
            },
        )
        .highlight_background_color(p.row_highlight);
    *habits_state.offset_mut() = 0;
    frame.render_stateful_widget(simple_list, area, habits_state);
}

pub fn render_habit_item(
    area: Rect,
    buf: &mut ratatui::buffer::Buffer,
    _is_selected: bool,
    habit_name: &str,
    is_completed: bool,
    p: Palette,
) {
    let horizontal_layout = Layout::horizontal([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Max(12),
    ])
    .spacing(1);
    let [status_area, habit_name_area, streaks_buttons_area_] = area.layout(&horizontal_layout);
    let status_symbol = if is_completed {
        Symbols::COMPLETED
    } else {
        Symbols::NOT_COMPLETED_CIRCLE
    };
    let status_color = if is_completed { p.accent } else { p.fg_dim };
    let status_span = Span::styled(status_symbol, Style::default().fg(status_color));
    let habit_span = Span::styled(habit_name, Style::default().fg(p.fg));
    let line = TextLine::from(vec![Span::raw(" "), habit_span]);

    let streak_count = if is_completed { "🔥 365" } else { "🔥 23" };
    let streak_symbols = TextLine::from(vec![
        Span::raw(" "),
        Span::styled(streak_count, Style::default().fg(p.amber)),
        Span::raw(" "),
        Span::styled(Symbols::EDIT_ICON, Style::default().fg(p.fg_dim)),
        Span::raw("  "),
        Span::styled(Symbols::DELETE_ICON, Style::default().fg(p.fg_dim)),
        Span::raw(" "),
    ])
    .right_aligned();
    Widget::render(&status_span, status_area, buf);
    render_line_with_ellipsis(&line, habit_name_area, buf);
    Widget::render(&streak_symbols, streaks_buttons_area_, buf);
}
fn render_line_with_ellipsis(line: &TextLine, area: Rect, buf: &mut Buffer) {
    Widget::render(line, area, buf);
    render_ellipsis_if_overflow(buf, area, line.width());
}

pub fn render_habits_header(frame: &mut Frame, area: Rect, p: Palette) {
    let text = Text::from(Span::styled(
        " + Add habit ",
        Style::default()
            .bg(p.heatmap[2])
            .fg(p.fg)
            .add_modifier(Modifier::BOLD),
    ))
    .right_aligned();
    frame.render_widget(text, area);
}

#[allow(dead_code)]
fn render_empty_state(frame: &mut Frame, area: Rect, text: &str, p: Palette) {
    let span = Span::from(text).style(Style::new().fg(p.accent));
    let text_len = text.len() as u16;
    let half_width = text_len / 2;
    let center_x = area.x + area.width / 2;
    let middle_rect = Rect {
        x: center_x.saturating_sub(half_width),
        y: area.y + area.height / 2,
        width: area.width,
        height: 1,
    };
    frame.render_widget(span, middle_rect);
    let frame_buffer_mut = frame.buffer_mut();
    for position in area.positions() {
        let style = Style::new().fg(p.fg_dim).dim();
        let cell_style = frame_buffer_mut[position].style();
        if let Some(fg_color) = cell_style.fg
            && fg_color != p.accent
        {
            frame_buffer_mut[position].set_symbol("⧸").set_style(style);
        }
    }
}

fn render_stats_column(
    app: &App,
    frame: &mut Frame,
    stats_area: Rect,
    goal_tile_state: &mut ListState,
    goal_row_state: &mut ListState,
) {
    let p = app.palette();
    let [cards_area, heatmap_area, streaks_area] = stats_area.layout(
        &Layout::vertical([
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Max(9),
        ])
        .spacing(1),
    );
    render_stat_cards(frame, cards_area, p);
    render_goal_progress(app, frame, heatmap_area, goal_tile_state, goal_row_state);
    render_top_streaks(frame, streaks_area, p);
}

fn render_stat_cards(frame: &mut Frame, area: Rect, p: Palette) {
    let [date_block_area, streak_count_block_area] = area.layout(
        &Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).spacing(1),
    );
    render_date_card(frame, date_block_area, p);
    render_streak_card(frame, streak_count_block_area, p);
}

fn render_date_card(frame: &mut Frame, area: Rect, p: Palette) {
    let date_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 0, 0));
    let date_block_inner_area = date_block.inner(area);
    frame.render_widget(date_block, area);
    let date_lines = vec![
        TextLine::from(vec![Span::styled("TODAY", Style::default().fg(p.fg_dim))]),
        TextLine::from(vec![Span::styled(
            "2024/06/05",
            Style::default().fg(p.accent).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(
        Paragraph::new(Text::from(date_lines)),
        date_block_inner_area,
    );
}

fn render_streak_card(frame: &mut Frame, area: Rect, p: Palette) {
    let streak_count_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 0, 0));
    let streak_count_block_inner_area = streak_count_block.inner(area);
    frame.render_widget(streak_count_block, area);
    let streak_count_lines = vec![
        TextLine::from(vec![Span::styled(
            "BEST STREAK",
            Style::default().fg(p.fg_dim),
        )]),
        TextLine::from(vec![Span::styled(
            "🔥 21 days",
            Style::default().fg(p.amber).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(
        Paragraph::new(Text::from(streak_count_lines)),
        streak_count_block_inner_area,
    );
}

fn render_goal_progress(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    goal_tile_state: &mut ListState,
    goal_row_state: &mut ListState,
) {
    let p = app.palette();
    let (border_color, text_color) = focus_colors(app.is_goal_progress_in_focus, p);
    let goal_progress = Block::default()
        .title(" goal progress ")
        .title_style(Style::new().fg(text_color))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(border_color))
        .padding(Padding::new(1, 1, 1, 0));
    let goal_progress_inner_area = goal_progress.inner(area);
    let vertical_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [goal_progress_header_area, goal_progress_list_area] =
        goal_progress_inner_area.layout(&vertical_layout);
    frame.render_widget(&goal_progress, area);
    render_goal_progress_header(app, frame, goal_progress_header_area, goal_tile_state);
    render_goal_progress_list(app, frame, goal_progress_list_area, goal_row_state);
}

fn render_goal_progress_list(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    goal_row_state: &mut ListState,
) {
    let p = app.palette();
    let simple_list =
        SimpleList::new(
            vec!["2", "2", "2"],
            |index, item_area, buf, is_selected| match index {
                0 => render_goal_progress_items(app, item_area, buf, is_selected),
                1 => render_goal_progress_items(app, item_area, buf, is_selected),
                2 => render_goal_progress_items(app, item_area, buf, is_selected),
                _ => {}
            },
        )
        .highlight_background_color(p.row_highlight)
        .render_line()
        .line_color(p.border);
    *goal_row_state.offset_mut() = 0;
    frame.render_stateful_widget(simple_list, area, goal_row_state);
}

fn render_goal_progress_items(app: &App, item_area: Rect, buf: &mut Buffer, is_selected: bool) {
    // Implementation for rendering individual goal progress items
    let vertical_layout =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).spacing(0);
    let [habit_name_area, progress_bar_area] = item_area.layout(&vertical_layout);
    render_goal_progress_habit_name(app, habit_name_area, buf, is_selected);
    render_goal_progress_bar(app, progress_bar_area, buf, is_selected);
}

fn render_goal_progress_habit_name(app: &App, area: Rect, buf: &mut Buffer, _is_selected: bool) {
    let p = app.palette();
    let habit_name = "Morning Run"; // Example habit name
    let habit_span = Span::styled(habit_name, Style::default().fg(p.fg));
    let line = TextLine::from(vec![Span::raw(" "), habit_span]);
    render_line_with_ellipsis(&line, area, buf);
}

fn render_goal_progress_bar(app: &App, area: Rect, buf: &mut Buffer, _is_selected: bool) {
    let p = app.palette();
    let horizontal_layout =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(5)]).spacing(1);
    let [progress_bar_area, percentage_area] = area.layout(&horizontal_layout);
    let empty_progress_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Thick)
        .border_style(Style::new().fg(app.palette().border));
    Widget::render(&empty_progress_block, progress_bar_area, buf);
    let completed_progress_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Thick)
        .border_style(Style::new().fg(p.heatmap[2]));
    let completed_area = Rect {
        x: progress_bar_area.x,
        y: progress_bar_area.y,
        width: (progress_bar_area.width as f32 * 0.75) as u16, // Example completion percentage
        height: progress_bar_area.height,
    };
    Widget::render(&completed_progress_block, completed_area, buf);
    let percentage = "100%"; // Example percentage
    let percentage_span = Span::styled(percentage, Style::default().fg(p.amber));

    Widget::render(&percentage_span, percentage_area, buf);
}

fn render_goal_progress_header(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    goal_tile_state: &mut ListState,
) {
    let p = app.palette();
    let selected = goal_tile_state.selected().unwrap_or(0);
    let tile_items: Vec<TileItem> = app
        .goal_progress_options
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
        .highlight_style(Style::new().fg(p.accent));
    frame.render_stateful_widget(tile_list, area, goal_tile_state);
}

fn render_top_streaks(frame: &mut Frame, area: Rect, p: Palette) {
    let top_streaks_block = Block::default()
        .title(" top streaks ")
        .title_style(Style::new().fg(p.fg_dim))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let top_streaks_inner_area = top_streaks_block.inner(area);
    frame.render_widget(top_streaks_block, area);

    let top_streaks_horizontal_layout =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(5)]).flex(Flex::SpaceBetween);
    let [left_streak, right_streak] = top_streaks_horizontal_layout.areas(top_streaks_inner_area);

    let mut top_streaks_line = String::new();
    for _ in 0..top_streaks_inner_area.width {
        top_streaks_line.push(HORIZONTAL.chars().next().unwrap());
    }

    let mut left_streaks_lines = vec![];
    let mut right_streak_lines = vec![];

    left_streaks_lines.push(TextLine::from(vec![
        Span::styled("#1", Style::default().fg(p.fg_dim)),
        Span::styled("   Morning Run", Style::default().fg(p.fg)),
    ]));
    right_streak_lines.push(TextLine::from(vec![Span::styled(
        "🔥 21",
        Style::default().fg(p.amber),
    )]));
    left_streaks_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(p.border)));
    right_streak_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(p.border)));

    left_streaks_lines.push(TextLine::from(vec![
        Span::styled("#2", Style::default().fg(p.fg_dim)),
        Span::styled("   No Sugar", Style::default().fg(p.fg)),
    ]));
    right_streak_lines.push(TextLine::from(vec![Span::styled(
        "🔥 40",
        Style::default().fg(p.amber),
    )]));
    left_streaks_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(p.border)));
    right_streak_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(p.border)));

    left_streaks_lines.push(TextLine::from(vec![
        Span::styled("#3", Style::default().fg(p.fg_dim)),
        Span::styled("   Read", Style::default().fg(p.fg)),
    ]));
    right_streak_lines.push(TextLine::from(vec![Span::styled(
        "🔥 16",
        Style::default().fg(p.amber),
    )]));

    frame.render_widget(Paragraph::new(Text::from(left_streaks_lines)), left_streak);
    frame.render_widget(Paragraph::new(Text::from(right_streak_lines)), right_streak);
}
