use crate::app::App;
use crate::palette::Palette;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::symbols::line::HORIZONTAL;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

pub fn render_dashboard(app: &mut App, frame: &mut Frame, app_area: Rect) {
    let p = app.palette();
    let horizontal =
        Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).spacing(1);
    let [dashboard_column, stats_column] = app_area.layout(&horizontal);
    let dashboard_block = Block::default()
        .title(" active habits ")
        .title_style(Style::new().fg(p.accent))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.accent))
        .padding(Padding::new(1, 1, 1, 1));
    let dashboard_inner_area = dashboard_block.inner(dashboard_column);
    frame.render_widget(dashboard_block, dashboard_column);
    render_empty_state(frame, dashboard_inner_area, " No active habits ", p);
    render_stats_column(app, frame, stats_column, p);
}

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
            Constraint::Length(6),
            Constraint::Fill(1),
            Constraint::Length(10),
        ])
        .spacing(1),
    );
    render_stat_cards(frame, cards_area, p);
    render_heatmap(frame, heatmap_area, p);
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
        .padding(Padding::new(1, 1, 1, 1));
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
        .padding(Padding::new(1, 1, 1, 1));
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

fn render_heatmap(frame: &mut Frame, area: Rect, p: Palette) {
    let heat_map_habits = Block::default()
        .title(" Aug 2026 ")
        .title_style(Style::new().fg(p.fg_dim))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::new(1, 1, 1, 1));
    frame.render_widget(heat_map_habits, area);
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
