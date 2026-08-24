use crate::palette::Palette;
use crossterm::event::KeyCode;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::ListState;
use ratatui::{buffer::Buffer, layout::Rect};

pub fn render_ellipsis_if_overflow(buf: &mut Buffer, area: Rect, content_width: usize) {
    if content_width as u16 > area.width && area.width > 0 {
        buf[(area.right() - 1, area.top())].set_symbol("…");
    }
}

pub fn focus_colors(is_focused: bool, p: Palette) -> (Color, Color) {
    if is_focused {
        (p.accent, p.accent)
    } else {
        (p.border, p.fg_dim)
    }
}

pub fn selection_modifier(state: &ListState, index: usize) -> Modifier {
    if state.selected() == Some(index) {
        Modifier::BOLD
    } else {
        Modifier::empty()
    }
}

pub fn is_right_key(code: KeyCode) -> bool {
    matches!(
        code,
        KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right
    )
}

pub fn is_left_key(code: KeyCode) -> bool {
    matches!(
        code,
        KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Left
    )
}

pub fn is_up_key(code: KeyCode) -> bool {
    matches!(code, KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up)
}

pub fn is_down_key(code: KeyCode) -> bool {
    matches!(
        code,
        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down
    )
}
