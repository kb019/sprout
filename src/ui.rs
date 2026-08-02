use crate::app::App;
use crate::palette::Palette;
use crate::sprout::{Sprout, SproutPercentage};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::canvas::{Canvas, Context, Painter};
use ratatui::widgets::{Block, Borders, Padding};
use ratatui::{Frame, symbols};

pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical: Layout =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [menu_column, _column_two, _column_three] = main.layout(&horizontal);

    render_menu_column(app, frame, menu_column);
    draw_app_name(frame, top);
}

pub fn render_menu_column(app: &mut App, frame: &mut Frame, menu_area: Rect) {
    let vertical_menu_rows: Layout = Layout::vertical([
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
    ])
    .spacing(1);
    let [sprout_box, navigate_box, summary_box] = menu_area.layout(&vertical_menu_rows);
    render_sprout(app, frame, sprout_box);
    render_navigate(app, frame, navigate_box);
    render_summary(app, frame, summary_box);
}

pub fn render_sprout(_app: &mut App, frame: &mut Frame, sprout_area: Rect) {
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

    let sprout_grow_percentage = SproutPercentage::TenPercent;

    let canvas_x_bounds = [-40.0, 40.0];
    let canvas_y_bounds = [-40.0, 40.0];
    let canvas = Canvas::default()
        .x_bounds(canvas_x_bounds)
        .y_bounds(canvas_y_bounds)
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&Sprout {
                percentage: sprout_grow_percentage,
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

pub fn render_dots(frame_buffer_mut: &mut Buffer, padding_bottom: u16, dot_area: Rect) {
    let dot_area_bottom_point = dot_area.bottom().saturating_sub(padding_bottom);
    let dot_area_left_point = dot_area.left();
    if !dot_area.is_empty() {
        for (_i, position) in dot_area.positions().enumerate() {
            let mut dot_style = Style::new().fg(Palette::DOT).add_modifier(Modifier::DIM);
            if position.y == dot_area_bottom_point && position.x >= dot_area_left_point {
                //make the last line of the sprout dots yellow
                dot_style = Style::new().fg(Palette::SOIL);
            }
            frame_buffer_mut[position]
                .set_symbol("·")
                .set_style(dot_style);
        }
    }
}

pub fn reset_color_plant_cells(
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
            // Braille packs a 2 (x) x 4 (y) sub-pixel grid into each terminal cell,
            // so collapse sub-pixel coords down to cell coords first.
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

pub fn render_navigate(_app: &mut App, frame: &mut Frame, navigate_area: Rect) {
    let navigate_block = Block::default()
        .title(" navigate ")
        .title_style(Style::new().fg(Palette::TEXT_SECONDARY))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER));
    frame.render_widget(navigate_block, navigate_area);
}

pub fn render_summary(_app: &mut App, frame: &mut Frame, summary_area: Rect) {
    let summary_block = Block::default()
        .title(" summary ")
        .title_style(Style::new().fg(Palette::TEXT_SECONDARY))
        .borders(Borders::ALL)
        .border_style(Style::new().fg(Palette::BORDER));
    frame.render_widget(summary_block, summary_area);
}

pub fn draw_app_name(frame: &mut Frame, area: Rect) {
    let title: TextLine<'_> = TextLine::from_iter([
        Span::from("sprout").style(Style::new().fg(Palette::BRAND_GREEN).bold()),
        Span::from(" - habit tracker").style(Style::new().fg(Palette::TEXT_SECONDARY)),
    ]);
    frame.render_widget(title.left_aligned(), area);
}
