use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::symbols::line::HORIZONTAL;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::app::App;
use crate::palette::Palette;
use crate::vendor::barchart::{Bar, BarChart};

pub fn render_stats_column(app: &mut App, frame: &mut Frame, stats_area: Rect) {
    let p = app.palette();
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let vertical_layout = Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).spacing(1);
    let [cards_area, data_area] = stats_area.layout(&vertical_layout);
    render_stat_cards(app, frame, cards_area, p);
    render_stat_data(app, frame, data_area, p);
    frame.render_widget(stats_block, stats_area);
}

pub fn render_stat_cards(_app: &mut App, frame: &mut Frame, area: Rect, p: Palette) {
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
    render_streak_card(frame, streak_card_area, p);
    render_week_card(frame, week_card_area, p);
    render_active_days_card(frame, active_days_card_area, p);
    render_habits_tracked_card(frame, habits_tracked_area, p);
}

pub fn render_streak_card(frame: &mut Frame, area: Rect, p: Palette) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let lines = vec![
        TextLine::from(vec![Span::styled(
            "BEST STREAK",
            Style::default().fg(p.fg_dim),
        )]),
        TextLine::from(vec![Span::styled(
            "🔥 21",
            Style::default().fg(p.amber).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_week_card(frame: &mut Frame, area: Rect, p: Palette) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let lines = vec![
        TextLine::from(vec![Span::styled(
            "WEEKLY AVERAGE",
            Style::default().fg(p.fg_dim),
        )]),
        TextLine::from(vec![Span::styled(
            "57%",
            Style::default().fg(p.accent).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_active_days_card(frame: &mut Frame, area: Rect, p: Palette) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let lines = vec![
        TextLine::from(vec![Span::styled(
            "ACTIVE DAYS (2026)",
            Style::default().fg(p.fg_dim),
        )]),
        TextLine::from(vec![Span::styled(
            "172",
            Style::default().fg(p.fg).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_habits_tracked_card(frame: &mut Frame, area: Rect, p: Palette) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let lines = vec![
        TextLine::from(vec![Span::styled(
            "HABITS TRACKED",
            Style::default().fg(p.fg_dim),
        )]),
        TextLine::from(vec![Span::styled(
            "5",
            Style::default().fg(p.fg).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_stat_data(_app: &mut App, frame: &mut Frame, area: Rect, p: Palette) {
    let horizontal_layout =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).spacing(0);
    let [bar_area, top_streaks_area] = area.layout(&horizontal_layout);
    render_bar_chart(frame, bar_area, p);
    render_top_streaks(frame, top_streaks_area, p);
}

pub fn render_bar_chart(frame: &mut Frame, area: Rect, p: Palette) {
    let bar_chart_block = Block::default()
        .title("  last 7 days · completion")
        .title_style(Style::new().fg(p.fg_dim))
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(2, 2, 1, 1));

    let bar_chart_inner_area = bar_chart_block.inner(area);

    let horizontal_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Percentage(80),
        Constraint::Fill(1),
    ]);
    let [_top_area, middle_area, _bottom_area] = bar_chart_inner_area.layout(&horizontal_layout);

    frame.render_widget(bar_chart_block, area);

    let bar_color = Style::new().fg(p.heatmap[2]);
    let today_bar_color = Style::new().fg(p.accent);
    let label_value_style = Style::new().fg(p.fg_dim);

    let bars = vec![
        Bar::with_label("Sun", 100).style(bar_color),
        Bar::with_label("Mon", 70).style(bar_color),
        Bar::with_label("Tue", 65).style(bar_color),
        Bar::with_label("Wed", 65).style(bar_color),
        Bar::with_label("Thu", 65).style(bar_color),
        Bar::with_label("Fri", 65).style(bar_color),
        Bar::with_label("Sat", 7).style(today_bar_color),
    ];

    let n = bars.len() as u16;
    if n == 0 {
        return;
    }

    const MAX_BAR_WIDTH: u16 = 6;
    const MIN_BAR_GAP: u16 = 1;

    let gap_count = n.saturating_sub(1);
    let minimum_gap_space = MIN_BAR_GAP.saturating_mul(gap_count);
    let available_for_bars = middle_area.width.saturating_sub(minimum_gap_space);
    let bar_width = (available_for_bars / n).clamp(1, MAX_BAR_WIDTH);
    let used_by_bars = bar_width.saturating_mul(n);
    let remaining_space = middle_area.width.saturating_sub(used_by_bars);
    let bar_gap = remaining_space
        .checked_div(gap_count)
        .map(|d| d.max(MIN_BAR_GAP))
        .unwrap_or(0);

    let bar_chart = BarChart::vertical(bars)
        .bar_width(bar_width)
        .bar_gap(bar_gap)
        .label_style(label_value_style)
        .value_style(label_value_style);

    frame.render_widget(bar_chart, middle_area);
}

pub fn render_top_streaks(frame: &mut Frame, area: Rect, p: Palette) {
    let top_streaks_block = Block::default()
        .title(" streak leaderboard ")
        .title_style(Style::new().fg(p.fg_dim))
        .borders(Borders::LEFT)
        .border_style(Style::new().fg(p.border))
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
        Span::styled("#1", Style::default().fg(p.amber)),
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
