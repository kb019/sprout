use crate::app::App;
use crate::state::States;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Fill, Padding};
mod add_modal;
mod delete_modal;
mod edit_modal;
mod progress_modal;
mod reset_modal;
pub fn render_modals(app: &mut App, frame: &mut Frame, states: &mut States) {
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
    if app.display_add_modal {
        add_modal::render_add_modal(app, frame, modal_inner_area, states);
    }

    if app.progress_modal_for_habit_id.is_some() {
        progress_modal::render_progress_modal(app, frame, modal_inner_area, states);
    }

    if app.display_delete_modal.is_some() {
        delete_modal::render_delete_modal(app, frame, modal_inner_area, states);
    }

    if app.display_edit_modal.is_some() {
        edit_modal::render_edit_modal(app, frame, modal_inner_area, states);
    }

    if app.display_reset_modal {
        reset_modal::render_reset_modal(app, frame, modal_inner_area, states);
    }
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
