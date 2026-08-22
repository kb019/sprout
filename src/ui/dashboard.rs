use crate::app::App;
use crate::palette::Palette;
use crate::symbols::Symbols;
use crate::widgets::simple_list::SimpleList;
use crate::widgets::tile_list::{TileItem, TileList, TileType};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::symbols::line::HORIZONTAL;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::{Block, Borders, ListState, Padding, Paragraph, Widget};
use unicode_width::UnicodeWidthStr;

pub fn render_dashboard(app: &mut App, frame: &mut Frame, app_area: Rect) {
    let p = app.palette();
    let horizontal =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).spacing(1);
    let [dashboard_column, stats_column] = app_area.layout(&horizontal);
    let mut border_color = p.border;
    let mut text_color = p.fg_dim;
    if app.is_dashboard_in_focus {
        border_color = p.accent;
        text_color = p.accent;
    }
    let dashboard_block = Block::default()
        .title(" active habits ")
        .title_style(Style::new().fg(text_color))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(border_color))
        .padding(Padding::new(1, 1, 1, 1));
    let dashboard_inner_area = dashboard_block.inner(dashboard_column);
    frame.render_widget(dashboard_block, dashboard_column);
    // render_empty_state(frame, dashboard_inner_area, " No active habits ", p);
    render_habits_content(app, frame, dashboard_inner_area);
    render_stats_column(app, frame, stats_column, p);
}

pub fn render_habits_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let vertical_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [header_area, habits_list_area] = area.layout(&vertical_layout);
    render_habits_header(frame, header_area, p);
    render_habits_list(app, frame, habits_list_area);
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn render_habits_list(app: &mut App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let simple_list =
        SimpleList::new(
            vec!["2", "2", "3"],
            |index, item_area, buf, is_selected| match index {
                0 => render_habit_item(item_area, buf, is_selected, "Morning Run", true, p),
                1 => render_habit_item(item_area, buf, is_selected, "No Sugar", false, p),
                2 => render_habit_item(item_area, buf, is_selected, "Read", true, p),
                _ => {}
            },
        );
    frame.render_stateful_widget(simple_list, area, &mut ListState::default());
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
        Constraint::Max(11),
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
        Span::styled(streak_count, Style::default().fg(p.amber)),
        Span::raw(" "),
        Span::styled(Symbols::EDIT_ICON, Style::default().fg(p.fg_dim)),
        Span::raw("  "),
        Span::styled(Symbols::DELETE_ICON, Style::default().fg(p.fg_dim)),
    ])
    .right_aligned();
    Widget::render(&status_span, status_area, buf);
    Widget::render(&line, habit_name_area, buf);
    let line_width = (1 + UnicodeWidthStr::width(habit_name)) as u16;
    if line_width > habit_name_area.width && habit_name_area.width > 0 {
        buf[(habit_name_area.right() - 1, habit_name_area.top())].set_symbol("…");
    }
    Widget::render(&streak_symbols, streaks_buttons_area_, buf);
}
pub fn render_habits_header(frame: &mut Frame, area: Rect, p: Palette) {
    let text = Text::from(Span::styled(
        " + Add habit ",
        Style::default()
            .bg(p.heatmap[2])
            .fg(p.fg_dim)
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

fn render_stats_column(_app: &mut App, frame: &mut Frame, stats_area: Rect, p: Palette) {
    let [cards_area, heatmap_area, streaks_area] = stats_area.layout(
        &Layout::vertical([
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(9),
        ])
        .spacing(1),
    );
    render_stat_cards(frame, cards_area, p);
    render_goal_progress(frame, heatmap_area, p);
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

fn render_goal_progress(frame: &mut Frame, area: Rect, p: Palette) {
    let goal_progress = Block::default()
        .title(" goal progress ")
        .title_style(Style::new().fg(p.fg_dim))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let goal_progress_inner_area = goal_progress.inner(area);
    let vertical_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [goal_progress_header_area, _goal_progress_list_area] =
        goal_progress_inner_area.layout(&vertical_layout);
    frame.render_widget(&goal_progress, area);
    render_goal_progress_header(frame, goal_progress_header_area, p);
    // render_goal_progress_list(frame, goal_progress_list_area, p);
}

fn render_goal_progress_header(frame: &mut Frame, area: Rect, p: Palette) {
    let tile_list = TileList::new(vec![
        TileItem::new(
            Text::from("Daily")
                .add_modifier(Modifier::BOLD)
                .bg(p.selection)
                .centered(),
        ),
        TileItem::new("Weekly"),
        TileItem::new("Monthly"),
        TileItem::new("Yearly"),
    ])
    .tile_type(TileType::Unbordered)
    .style(Style::new().fg(p.fg_dim))
    .highlight_style(Style::new().fg(p.accent));
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    frame.render_stateful_widget(tile_list, area, &mut list_state);
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
