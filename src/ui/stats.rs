use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::symbols::line::HORIZONTAL;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::{Bar, BarChart, Block, Borders, Padding, Paragraph};

use crate::app::App;
use crate::palette::Palette;

pub fn render_stats_column(app: &mut App, frame: &mut Frame, stats_area: Rect) {
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 1, 1, 1));
    let vertical_layout = Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).spacing(1);
    let [cards_area, data_area] = stats_area.layout(&vertical_layout);
    render_stat_cards(app, frame, cards_area);
    render_stat_data(app, frame, data_area);
    frame.render_widget(stats_block, stats_area);
}

pub fn render_stat_cards(_app: &mut App, frame: &mut Frame, area: Rect) {
    let horizontal_layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .flex(Flex::SpaceBetween)
    .spacing(1);
    let [
        streak_card_area,
        week_card_area,
        active_days_card_area,
        habits_tracked_area,
    ] = area.layout(&horizontal_layout);
    render_streak_card(frame, streak_card_area);
    render_week_card(frame, week_card_area);
    render_active_days_card(frame, active_days_card_area);
    render_habits_tracked_card(frame, habits_tracked_area);
}

pub fn render_streak_card(frame: &mut Frame, area: Rect) {
    let streak_count_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 1, 1, 1));
    let streak_count_block_inner_area = streak_count_block.inner(area);
    frame.render_widget(streak_count_block, area);
    let mut streak_count_lines = vec![];
    streak_count_lines.push(TextLine::from(vec![Span::styled(
        "BEST STREAK",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));
    streak_count_lines.push(TextLine::from(vec![Span::styled(
        "🔥 21",
        Style::default()
            .fg(Palette::AMBER)
            .add_modifier(Modifier::BOLD),
    )]));
    frame.render_widget(
        Paragraph::new(Text::from(streak_count_lines)),
        streak_count_block_inner_area,
    );
}

pub fn render_week_card(frame: &mut Frame, area: Rect) {
    let week_count_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 1, 1, 1));
    let week_count_block_inner_area = week_count_block.inner(area);
    frame.render_widget(week_count_block, area);
    let mut week_count_lines = vec![];
    week_count_lines.push(TextLine::from(vec![Span::styled(
        "WEEKLY AVERAGE",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));
    week_count_lines.push(TextLine::from(vec![Span::styled(
        "57%",
        Style::default()
            .fg(Palette::BRAND_GREEN)
            .add_modifier(Modifier::BOLD),
    )]));
    frame.render_widget(
        Paragraph::new(Text::from(week_count_lines)),
        week_count_block_inner_area,
    );
}

pub fn render_active_days_card(frame: &mut Frame, area: Rect) {
    let active_days_count_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 1, 1, 1));
    let active_days_count_block_inner_area = active_days_count_block.inner(area);
    frame.render_widget(active_days_count_block, area);
    let mut active_days_count_lines = vec![];
    active_days_count_lines.push(TextLine::from(vec![Span::styled(
        "ACTIVE DAYS (2026)",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));
    active_days_count_lines.push(TextLine::from(vec![Span::styled(
        "172",
        Style::default()
            .fg(Palette::TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD),
    )]));
    frame.render_widget(
        Paragraph::new(Text::from(active_days_count_lines)),
        active_days_count_block_inner_area,
    );
}

pub fn render_habits_tracked_card(frame: &mut Frame, area: Rect) {
    let habits_tracked_count_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 1, 1, 1));
    let habits_tracked_count_block_inner_area = habits_tracked_count_block.inner(area);
    frame.render_widget(habits_tracked_count_block, area);
    let mut habits_tracked_count_lines = vec![];
    habits_tracked_count_lines.push(TextLine::from(vec![Span::styled(
        "HABITS TRACKED",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));
    habits_tracked_count_lines.push(TextLine::from(vec![Span::styled(
        "5",
        Style::default()
            .fg(Palette::TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD),
    )]));
    frame.render_widget(
        Paragraph::new(Text::from(habits_tracked_count_lines)),
        habits_tracked_count_block_inner_area,
    );
}

pub fn render_stat_data(_app: &mut App, frame: &mut Frame, area: Rect) {
    let horizontal_layout =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).spacing(0);
    let [bar_area, top_streaks_area] = area.layout(&horizontal_layout);
    render_bar_chart(frame, bar_area);
    render_top_streaks(frame, top_streaks_area);
}

pub fn render_bar_chart(frame: &mut Frame, area: Rect) {
    let bar_chart_block = Block::default()
        .title_style(Style::new().fg(Palette::BORDER))
        .title("  last 7 days · completion")
        .title_style(Style::new().fg(Palette::TEXT_SECONDARY))
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(2, 2, 1, 1));
    let bar_chart_inner_area = bar_chart_block.inner(area);
    let horizontal_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Percentage(80),
        Constraint::Fill(1),
    ]);
    let [_top_area, middle_area, _bottom_area] = bar_chart_inner_area.layout(&horizontal_layout);
    frame.render_widget(bar_chart_block, area);
    let bar_color = Style::new().fg(Palette::HEATMAP_2);
    let todaay_bar_color = Style::new().fg(Palette::BRAND_GREEN);
    let label_value_style = Style::new().fg(Palette::TEXT_SECONDARY);
    let bars = vec![
        Bar::with_label("Sun", 100).style(bar_color),
        Bar::with_label("Mon", 70).style(bar_color),
        Bar::with_label("Tue", 65).style(bar_color),
        Bar::with_label("Wed", 65).style(bar_color),
        Bar::with_label("Thu", 65).style(bar_color),
        Bar::with_label("Fri", 65).style(bar_color),
        Bar::with_label("Sat", 7).style(todaay_bar_color),
    ];
    let bar_width = 6;
    let bar_gap = (middle_area
        .width
        .saturating_sub(bar_width * bars.len() as u16))
        / (bars.len() as u16 - 1);
    let mut bar_chart = BarChart::vertical(bars).bar_width(bar_width);
    bar_chart = bar_chart
        .bar_gap(bar_gap.max(1))
        .label_style(label_value_style)
        .value_style(label_value_style);

    frame.render_widget(bar_chart, middle_area);
}

pub fn render_top_streaks(frame: &mut Frame, area: Rect) {
    let top_streaks_block = Block::default()
        .title(" streak leaderboard ")
        .title_style(Style::new().fg(Palette::TEXT_SECONDARY))
        .borders(Borders::LEFT)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 2, 1, 1));
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
        Span::styled("#1", Style::default().fg(Palette::AMBER)),
        Span::styled("   Morning Run", Style::default().fg(Palette::TEXT_PRIMARY)),
    ]));
    right_streak_lines.push(TextLine::from(vec![Span::styled(
        "🔥 21",
        Style::default().fg(Palette::AMBER),
    )]));
    left_streaks_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(Palette::BORDER)));
    right_streak_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(Palette::BORDER)));

    left_streaks_lines.push(TextLine::from(vec![
        Span::styled("#2", Style::default().fg(Palette::TEXT_SECONDARY)),
        Span::styled("   No Sugar", Style::default().fg(Palette::TEXT_PRIMARY)),
    ]));
    right_streak_lines.push(TextLine::from(vec![Span::styled(
        "🔥 40",
        Style::default().fg(Palette::AMBER),
    )]));
    left_streaks_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(Palette::BORDER)));
    right_streak_lines
        .push(TextLine::from(top_streaks_line.clone()).style(Style::default().fg(Palette::BORDER)));

    left_streaks_lines.push(TextLine::from(vec![
        Span::styled("#3", Style::default().fg(Palette::TEXT_SECONDARY)),
        Span::styled("   Read", Style::default().fg(Palette::TEXT_PRIMARY)),
    ]));
    right_streak_lines.push(TextLine::from(vec![Span::styled(
        "🔥 16",
        Style::default().fg(Palette::AMBER),
    )]));

    frame.render_widget(Paragraph::new(Text::from(left_streaks_lines)), left_streak);
    frame.render_widget(Paragraph::new(Text::from(right_streak_lines)), right_streak);
}
