use chrono::{Datelike, Local};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget};

use crate::app::App;
use crate::palette::Palette;
use crate::state::States;
use crate::utils::{focus_colors, progress, render_ellipsis_if_overflow};
use crate::vendor::barchart::{Bar, BarChart};
use crate::widgets::simple_list::SimpleList;

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn render_stats_column(
    app: &mut App,
    frame: &mut Frame,
    stats_area: Rect,
    states: &mut States,
) {
    let p = app.palette();
    let stats_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let vertical_layout = Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).spacing(1);
    let [cards_area, data_area] = stats_area.layout(&vertical_layout);
    render_stat_cards(app, frame, cards_area, p, states);
    render_stat_data(app, frame, data_area, p, states);
    frame.render_widget(stats_block, stats_area);
}

#[allow(clippy::needless_pass_by_ref_mut)]
pub fn render_stat_cards(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    p: Palette,
    states: &mut States,
) {
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
    render_streak_card(app, frame, streak_card_area, p, states);
    render_week_card(app, frame, week_card_area, p, states);
    render_active_days_card(app, frame, active_days_card_area, p, states);
    render_habits_tracked_card(app, frame, habits_tracked_area, p);
}

pub fn render_streak_card(app: &App, frame: &mut Frame, area: Rect, p: Palette, states: &States) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let best_count = app.best_streaks.iter().map(|s| s.count).max().unwrap_or(0);
    let is_loading = states.best_streaks_state.is_fetching();
    let initial_span = if is_loading {
        Span::styled(format!("{} ", progress(app)), Style::default().fg(p.amber))
    } else {
        Span::raw("")
    };
    let lines = vec![
        TextLine::from(vec![
            initial_span,
            Span::styled("BEST STREAK", Style::default().fg(p.fg_dim)),
        ]),
        TextLine::from(vec![Span::styled(
            format!("🔥 {}", best_count),
            Style::default().fg(p.amber).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_week_card(app: &App, frame: &mut Frame, area: Rect, p: Palette, states: &States) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let is_loading = states.weekly_average_state.is_fetching();
    let days_elapsed = Local::now().weekday().num_days_from_monday() + 1;
    let avg = app.weekly_completion[..days_elapsed as usize]
        .iter()
        .sum::<u32>()
        / days_elapsed;
    let initial_span = if is_loading {
        Span::styled(format!("{} ", progress(app)), Style::default().fg(p.amber))
    } else {
        Span::raw("")
    };
    let lines = vec![
        TextLine::from(vec![
            initial_span,
            Span::styled("WEEKLY AVERAGE", Style::default().fg(p.fg_dim)),
        ]),
        TextLine::from(vec![Span::styled(
            format!("{}%", avg),
            Style::default().fg(p.accent).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_active_days_card(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    p: Palette,
    states: &States,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let year = Local::now().format("%Y").to_string();
    let is_loading = states.active_days_state.is_fetching();
    let initial_span = if is_loading {
        Span::styled(format!("{} ", progress(app)), Style::default().fg(p.amber))
    } else {
        Span::raw("")
    };
    let lines = vec![
        TextLine::from(vec![
            initial_span,
            Span::styled(
                format!("ACTIVE DAYS ({})", year),
                Style::default().fg(p.fg_dim),
            ),
        ]),
        TextLine::from(vec![Span::styled(
            app.active_days.to_string(),
            Style::default().fg(p.fg).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_habits_tracked_card(app: &App, frame: &mut Frame, area: Rect, p: Palette) {
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
            app.habits.len().to_string(),
            Style::default().fg(p.fg).add_modifier(Modifier::BOLD),
        )]),
    ];
    frame.render_widget(Paragraph::new(Text::from(lines)), inner);
}

pub fn render_stat_data(app: &App, frame: &mut Frame, area: Rect, p: Palette, states: &mut States) {
    let horizontal_layout =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).spacing(0);
    let [bar_area, top_streaks_area] = area.layout(&horizontal_layout);
    render_bar_chart(app, frame, bar_area, p, states);
    render_top_streaks(app, frame, top_streaks_area, p, states);
}

pub fn render_bar_chart(app: &App, frame: &mut Frame, area: Rect, p: Palette, states: &States) {
    let title = if states.weekly_average_state.is_fetching() {
        TextLine::from_iter([
            Span::raw("  "),
            Span::styled(format!("{} ", progress(app)), Style::new().fg(p.amber)),
            Span::styled("this week · completion %", Style::new().fg(p.fg_dim)),
        ])
    } else {
        TextLine::from(Span::styled(
            "  this week · completion %",
            Style::new().fg(p.fg_dim),
        ))
    };
    let bar_chart_block = Block::default()
        .title(title)
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

    const DAY_LABELS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let today_idx = Local::now().weekday().num_days_from_monday() as usize;

    let bars: Vec<Bar> = app
        .weekly_completion
        .iter()
        .enumerate()
        .map(|(i, &pct)| {
            let style = if i == today_idx {
                today_bar_color
            } else {
                bar_color
            };
            Bar::with_label(DAY_LABELS[i], pct as u64).style(style)
        })
        .collect();

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

pub fn render_top_streaks(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    p: Palette,
    states: &mut States,
) {
    let (border_color, title_color) = focus_colors(app.is_streak_leaderboard_in_focus, p);
    let top_streaks_block = Block::default()
        .title(" streak leaderboard ")
        .title_style(Style::new().fg(title_color))
        .borders(Borders::LEFT)
        .border_style(Style::new().fg(border_color))
        .padding(Padding::new(1, 2, 1, 1));
    let inner_area = top_streaks_block.inner(area);
    frame.render_widget(top_streaks_block, area);

    let mut ranked: Vec<(&str, i32)> = app
        .habits
        .iter()
        .filter_map(|h| {
            let s = app.active_streaks.get(&h.id).unwrap_or(&0);
            if s > &0 {
                Some((h.name.as_str(), *s))
            } else {
                None
            }
        })
        .collect();
    ranked.sort_by_key(|b| std::cmp::Reverse(b.1));

    if ranked.is_empty() {
        let empty = Text::from(TextLine::from(" No streaks yet").style(Style::new().fg(p.fg_dim)));
        frame.render_widget(empty, inner_area);
        return;
    }

    let heights: Vec<&str> = vec!["1"; ranked.len()];
    let list = SimpleList::new(heights, move |index, item_area, buf, _| {
        if let Some((name, streak)) = ranked.get(index) {
            let rank = format!("#{} ", index + 1);
            let streak_str = format!("🔥 {}", streak);
            let streak_len = streak_str.chars().count() as u16 + 1;
            let [name_area, streak_area] = item_area.layout(&Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(streak_len),
            ]));
            let name_line = TextLine::from(vec![
                Span::styled(rank, Style::default().fg(p.fg_dim)),
                Span::styled(*name, Style::default().fg(p.fg)),
            ]);
            render_line_with_ellipsis(&name_line, name_area, buf);
            Widget::render(
                TextLine::from(Span::styled(streak_str, Style::default().fg(p.amber)))
                    .right_aligned(),
                streak_area,
                buf,
            );
        }
    })
    .highlight_background_color(p.row_highlight)
    .highlight_symbol_color(p.accent)
    .parent_in_focus(app.is_streak_leaderboard_in_focus)
    .render_line()
    .line_color(p.border);

    let list_state = states.app_state.streak_leaderboard_state_mut();
    *list_state.offset_mut() = 0;
    frame.render_stateful_widget(list, inner_area, list_state);
}

fn render_line_with_ellipsis(line: &TextLine, area: Rect, buf: &mut Buffer) {
    Widget::render(line, area, buf);
    render_ellipsis_if_overflow(buf, area, line.width());
}
