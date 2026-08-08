use crate::app::App;
use crate::palette::Palette;
use crate::sprout::{Sprout, SproutPercentage};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::canvas::{Canvas, Context, Painter};
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
};
use ratatui::{Frame, symbols};

pub fn render_menu_column(
    app: &mut App,
    frame: &mut Frame,
    menu_area: Rect,
    list_state: &mut ListState,
) {
    let vertical_menu_rows: Layout = Layout::vertical([
        Constraint::Percentage(35),
        Constraint::Percentage(25),
        Constraint::Percentage(40),
    ])
    .spacing(1);
    let [sprout_box, navigate_box, summary_box] = menu_area.layout(&vertical_menu_rows);
    render_sprout(app, frame, sprout_box);
    render_navigate(app, frame, navigate_box, list_state);
    render_summary(app, frame, summary_box);
}

fn render_navigate(
    _app: &mut App,
    frame: &mut Frame,
    navigate_area: Rect,
    list_state: &mut ListState,
) {
    let navigate_block = Block::default()
        .title(" navigate ")
        .title_style(Style::new().fg(Palette::TEXT_SECONDARY))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 1, 1, 1));
    let menu_area = navigate_block.inner(navigate_area);
    frame.render_widget(navigate_block, navigate_area);
    render_menu(_app, frame, menu_area, list_state);
}

fn render_menu(app: &App, frame: &mut Frame, menu_area: Rect, list_state: &mut ListState) {
    let mut menu_items = vec![];

    for (i, menu_item) in app.menu.iter().enumerate() {
        let mut bold_modifier = Modifier::empty();

        if let Some(select) = list_state.selected()
            && select == i
        {
            bold_modifier = Modifier::BOLD;
        }
        let text = Text::from(*menu_item).add_modifier(bold_modifier);
        let item = ListItem::new(text);
        menu_items.push(item);
    }
    let list = List::new(menu_items)
        .style(Palette::TEXT_SECONDARY)
        .highlight_style(Style::new().fg(Palette::BRAND_GREEN).bg(Palette::SELECTION))
        .highlight_symbol("▍ ")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, menu_area, list_state);
}

fn render_summary(_app: &mut App, frame: &mut Frame, summary_area: Rect) {
    let summary_block = Block::default()
        .title(" summary ")
        .title_style(Style::new().fg(Palette::TEXT_SECONDARY))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(1, 1, 1, 1));
    let summary_inner_area = summary_block.inner(summary_area);

    let mut lines = vec![];
    lines.push(TextLine::from(vec![Span::styled(
        "TODAY ",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));
    lines.push(TextLine::from(vec![
        Span::styled("3/5 done", Style::default().fg(Palette::TEXT_PRIMARY))
            .add_modifier(Modifier::BOLD),
        Span::styled("    60%", Style::default().fg(Palette::TEXT_PRIMARY)),
    ]));
    lines.push(TextLine::from(vec![Span::from("")]));
    lines.push(TextLine::from(vec![Span::styled(
        "██████████░░░░░░",
        Style::default().fg(Palette::BRAND_GREEN),
    )]));
    lines.push(TextLine::from(vec![Span::from("")]));
    lines.push(TextLine::from(vec![Span::styled(
        "BEST STREAK",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));
    lines.push(TextLine::from(vec![Span::styled(
        "🔥 21 days",
        Style::default().fg(Palette::AMBER),
    )]));
    lines.push(TextLine::from(vec![Span::from("")]));
    lines.push(TextLine::from(vec![Span::styled(
        "Habits tracked",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));
    lines.push(TextLine::from(vec![Span::styled(
        "5",
        Style::default().fg(Palette::TEXT_SECONDARY),
    )]));

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, summary_inner_area);
    frame.render_widget(summary_block, summary_area);
}

fn render_sprout(app: &App, frame: &mut Frame, sprout_area: Rect) {
    let padding_left = 1;
    let padding_right = 1;
    let padding_top = 1;
    let padding_bottom = 1;
    let sprout_block = Block::default()
        .title(" sprout ")
        .title_style(Style::new().fg(Palette::BRAND_GREEN))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER))
        .padding(Padding::new(
            padding_left,
            padding_right,
            padding_top,
            padding_bottom,
        ));

    let sprout_grow_percentage = SproutPercentage::from_value(app.counter);
    let app_ref: &App = app;

    let canvas_x_bounds = [-40.0, 40.0];
    let canvas_y_bounds = [-40.0, 40.0];
    let canvas = Canvas::default()
        .x_bounds(canvas_x_bounds)
        .y_bounds(canvas_y_bounds)
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&Sprout {
                percentage: sprout_grow_percentage,
                app: app_ref,
            });
        });

    let dot_area = sprout_block.inner(sprout_area);
    let frame_buffer_mut = frame.buffer_mut();
    let dot_area = dot_area.intersection(*frame_buffer_mut.area());

    render_dots(frame_buffer_mut, padding_bottom, dot_area);

    frame.render_widget(sprout_block, sprout_area);
    frame.render_widget(canvas, dot_area);

    if !dot_area.is_empty() {
        let sprout = Sprout {
            percentage: sprout_grow_percentage,
            app: app_ref,
        };

        let points = sprout.percentage.data(&sprout);
        reset_color_plant_cells(
            dot_area,
            frame,
            canvas_x_bounds,
            canvas_y_bounds,
            points.tree,
            Palette::BRAND_GREEN,
        );
        reset_color_plant_cells(
            dot_area,
            frame,
            canvas_x_bounds,
            canvas_y_bounds,
            points.flower,
            Palette::AMBER,
        );
    }
}

fn render_dots(frame_buffer_mut: &mut Buffer, padding_bottom: u16, dot_area: Rect) {
    let dot_area_bottom_point = dot_area.bottom().saturating_sub(padding_bottom);
    let dot_area_left_point = dot_area.left();
    if !dot_area.is_empty() {
        for position in dot_area.positions() {
            let mut dot_style = Style::new().fg(Palette::TEXT_SECONDARY).dim();
            if position.y == dot_area_bottom_point && position.x >= dot_area_left_point {
                dot_style = Style::new().fg(Palette::SOIL);
            }
            frame_buffer_mut[position]
                .set_symbol("·")
                .set_style(dot_style);
        }
    }
}

fn reset_color_plant_cells(
    dot_area: Rect,
    frame: &mut Frame,
    canvas_x_bounds: [f64; 2],
    canvas_y_bounds: [f64; 2],
    points: Vec<(f64, f64)>,
    color: Color,
) {
    let mut ctx = Context::new(
        dot_area.width,
        dot_area.height,
        canvas_x_bounds,
        canvas_y_bounds,
        symbols::Marker::Braille,
    );
    let painter = Painter::from(&mut ctx);

    let frame_buffer_mut = frame.buffer_mut();
    for &(x, y) in &points {
        if let Some(p) = painter.get_point(x, y) {
            let cell_x = (p.0 / 2) as u16;
            let cell_y = (p.1 / 4) as u16;
            let pos = Position::new(dot_area.x + cell_x, dot_area.y + cell_y);

            if frame_buffer_mut.area().contains(pos) {
                frame_buffer_mut[pos].set_style(
                    Style::new()
                        .fg(color)
                        .remove_modifier(Modifier::DIM)
                        .add_modifier(Modifier::BOLD),
                );
            }
        }
    }
}
