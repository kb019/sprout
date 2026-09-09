use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{ListState, StatefulWidget},
};

const HIGHLIGHT_SYMBOL: &str = "▶  ";
const HIGHLIGHT_SYMBOL_WIDTH: u16 = 3;

pub struct SimpleList<'a, F>
where
    F: FnMut(usize, Rect, &mut Buffer, bool),
{
    background_color: Color,
    line_color: Color,
    symbol_color: Color,
    constraint_lengths: Vec<&'a str>,
    render_line: bool,
    show_highlight_symbol: bool,
    is_parent_in_focus: bool,
    item_draw_callback: F,
}

impl<'a, F> SimpleList<'a, F>
where
    F: FnMut(usize, Rect, &mut Buffer, bool),
{
    pub fn new(constraint_lengths: Vec<&'a str>, callback: F) -> Self {
        Self {
            background_color: Color::default(),
            line_color: Color::default(),
            symbol_color: Color::default(),
            constraint_lengths,
            item_draw_callback: callback,
            render_line: false,
            show_highlight_symbol: true,
            is_parent_in_focus: false,
        }
    }

    pub fn highlight_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn constraint_lengths(mut self, lengths: Vec<&'a str>) -> Self {
        self.constraint_lengths = lengths;
        self
    }

    pub fn render_line(mut self) -> Self {
        self.render_line = true;
        self
    }

    pub fn line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    pub fn disable_highlight_symbol(mut self) -> Self {
        self.show_highlight_symbol = false;
        self
    }

    pub fn highlight_symbol_color(mut self, color: Color) -> Self {
        self.symbol_color = color;
        self
    }

    pub fn parent_in_focus(mut self, focused: bool) -> Self {
        self.is_parent_in_focus = focused;
        self
    }
}

impl<'a, F> StatefulWidget for SimpleList<'a, F>
where
    F: FnMut(usize, Rect, &mut Buffer, bool),
{
    type State = ListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(&mut self, area, buf, state);
    }
}

impl<'a, F> StatefulWidget for &mut SimpleList<'a, F>
where
    F: FnMut(usize, Rect, &mut Buffer, bool),
{
    type State = ListState;
    // some of the logic is referred from ratatui::widgets::List
    #[allow(clippy::needless_range_loop)]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let spacing: u16 = if self.render_line { 1 } else { 0 };
        let symbol_width = if self.show_highlight_symbol {
            HIGHLIGHT_SYMBOL_WIDTH
        } else {
            0
        };

        let heights: Vec<u16> = self
            .constraint_lengths
            .iter()
            .map(|s| s.parse::<u16>().unwrap_or(0))
            .collect();

        if heights.is_empty() || area.is_empty() {
            return;
        }

        if state.selected().is_some_and(|s| s >= heights.len()) {
            state.select(Some(heights.len().saturating_sub(1)));
        }

        let (first, last) = get_item_bounds(
            &heights,
            state.selected(),
            state.offset(),
            area.height,
            spacing,
        );

        *state.offset_mut() = first;

        // Truncated fallback: selected item taller than entire area
        if last == first {
            let index = state.selected().unwrap_or(0);
            if let Some(&item_height) = heights.get(index) {
                let item_rect = Rect::new(
                    area.left(),
                    area.top(),
                    area.width,
                    area.height.min(item_height),
                )
                .intersection(area);
                if area.intersects(item_rect) {
                    if self.is_parent_in_focus && self.background_color != Color::default() {
                        buf.set_style(item_rect, Style::default().bg(self.background_color));
                    }
                    let callback_rect = if self.is_parent_in_focus {
                        shrink_left(item_rect, symbol_width)
                    } else {
                        item_rect
                    };
                    (self.item_draw_callback)(index, callback_rect, buf, true);
                    if self.is_parent_in_focus {
                        draw_symbol(
                            buf,
                            item_rect,
                            self.background_color,
                            self.symbol_color,
                            self.show_highlight_symbol,
                        );
                    }
                }
            }
            return;
        }

        let mut current_y = area.top();
        for i in first..last {
            let item_height = heights[i];
            let item_rect =
                Rect::new(area.left(), current_y, area.width, item_height).intersection(area);

            let is_selected = state.selected().is_some_and(|s| s == i);

            let callback_rect =
                if is_selected && self.is_parent_in_focus && self.show_highlight_symbol {
                    shrink_left(item_rect, symbol_width)
                } else {
                    item_rect
                };

            (self.item_draw_callback)(i, callback_rect, buf, is_selected);
            current_y += item_height;

            if is_selected && self.is_parent_in_focus && self.background_color != Color::default() {
                buf.set_style(item_rect, Style::default().bg(self.background_color));
            }

            if is_selected && self.is_parent_in_focus {
                draw_symbol(
                    buf,
                    item_rect,
                    self.background_color,
                    self.symbol_color,
                    self.show_highlight_symbol,
                );
            }

            // Divider between items, not after the last visible one
            if self.render_line && i < last - 1 {
                if current_y < area.bottom() {
                    buf.set_string(
                        area.left(),
                        current_y,
                        "─".repeat(area.width as usize),
                        Style::default().fg(self.line_color),
                    );
                }
                current_y += spacing;
            }
        }
    }
}

fn shrink_left(rect: Rect, amount: u16) -> Rect {
    Rect {
        x: rect.x.saturating_add(amount),
        width: rect.width.saturating_sub(amount),
        ..rect
    }
}

fn draw_symbol(buf: &mut Buffer, item_rect: Rect, bg: Color, fg: Color, enabled: bool) {
    if !enabled || item_rect.width < HIGHLIGHT_SYMBOL_WIDTH {
        return;
    }
    let mut style = Style::default();
    if bg != Color::default() {
        style = style.bg(bg);
    }
    if fg != Color::default() {
        style = style.fg(fg);
    }
    buf.set_string(item_rect.left(), item_rect.top(), HIGHLIGHT_SYMBOL, style);
}

fn get_item_bounds(
    heights: &[u16],
    selected: Option<usize>,
    offset: usize,
    max_height: u16,
    spacing: u16,
) -> (usize, usize) {
    let offset = offset.min(heights.len().saturating_sub(1));
    let mut first = offset;
    let mut last = offset;
    let mut height = 0u16;

    // Fill visible window starting from offset
    for &h in heights.iter().skip(offset) {
        if height + h > max_height {
            break;
        }
        height = height.saturating_add(h + spacing);
        last += 1;
    }

    let index_to_display = selected.unwrap_or(offset);

    // Scroll down to bring selected into view
    while index_to_display >= last && last < heights.len() {
        height = height.saturating_add(heights[last] + spacing);
        last += 1;
        while height > max_height && first < last {
            height = height.saturating_sub(heights[first] + spacing);
            first += 1;
        }
    }

    // Scroll up to bring selected into view
    while index_to_display < first && first > 0 {
        first -= 1;
        height = height.saturating_add(heights[first] + spacing);
        while height > max_height && last > first + 1 {
            last -= 1;
            height = height.saturating_sub(heights[last] + spacing);
        }
    }

    (first, last)
}
