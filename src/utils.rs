use ratatui::{buffer::Buffer, layout::Rect};

pub fn render_ellipsis_if_overflow(buf: &mut Buffer, area: Rect, content_width: usize) {
    if content_width as u16 > area.width && area.width > 0 {
        buf[(area.right() - 1, area.top())].set_symbol("…");
    }
}
