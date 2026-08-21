use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, BlockExt, BorderType, Borders, ListState, StatefulWidget, Widget};

use crate::widgets::tile_list::{TileBorderType, TileDirection, TileItem, TileList, TileType};

fn render_content(item: &TileItem, area: Rect, buf: &mut Buffer) {
    Widget::render(&item.content, area, buf);
    if item.width() as u16 > area.width && area.width > 0 {
        buf[(area.right() - 1, area.top())].set_symbol("…");
    }
}

impl Widget for TileList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(&self, area, buf, &mut state);
    }
}

impl Widget for &TileList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl StatefulWidget for TileList<'_> {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(&self, area, buf, state);
    }
}

impl StatefulWidget for &TileList<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        self.block.as_ref().render(area, buf);
        let list_area = self.block.inner_if_some(area);

        // Bordered:  → 3 rows tall, 2 extra cols (for left and right borders)
        // Unbordered: 1 row tall, no extra cols
        let (extra_width, required_height) = match self.tile_type {
            TileType::Bordered => (2u16, 3u16),
            TileType::Unbordered => (2u16, 1u16),
        };

        if list_area.is_empty() || list_area.height < required_height || self.items.is_empty() {
            return;
        }

        if state.selected().is_some_and(|s| s >= self.items.len()) {
            state.select(Some(self.items.len().saturating_sub(1)));
        }

        let (first_visible, last_visible) = self.get_tile_bounds(
            state.selected(),
            state.offset(),
            list_area.width as usize,
            extra_width as usize,
        );

        let mut current_width: u16 = 0;
        for (i, item) in self
            .items
            .iter()
            .enumerate()
            .skip(first_visible)
            .take(last_visible - first_visible)
        {
            let tile_width = item.width() as u16 + extra_width;
            let x = match self.direction {
                TileDirection::RightToLeft => {
                    current_width += tile_width + 1;
                    list_area.right().saturating_sub(current_width - 1)
                }
                TileDirection::LeftToRight => {
                    let x = list_area.left() + current_width;
                    current_width += tile_width + 1;
                    x
                }
            };
            let row_area = Rect::new(x, list_area.top(), tile_width, required_height);

            let is_selected = state.selected() == Some(i);
            let border_style = if is_selected {
                self.highlight_style
            } else {
                self.style
            };

            match self.tile_type {
                TileType::Bordered => {
                    let border_type = match self.tile_border_type {
                        TileBorderType::Rounded => BorderType::Rounded,
                        TileBorderType::Sharp => BorderType::Plain,
                    };
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .border_style(border_style)
                        .border_type(border_type);
                    let inner = block.inner(row_area);
                    render_content(item, inner, buf);
                    block.render(row_area, buf);
                }
                TileType::Unbordered => {
                    render_content(item, row_area, buf);
                }
            }

            if is_selected {
                buf.set_style(row_area, self.highlight_style);
            }
        }

        // When the selected tile is wider than the available area, render it truncated at full width
        if last_visible == first_visible {
            let tile_index = state.selected().unwrap_or(0);
            if let Some(item) = self.items.get(tile_index) {
                let tile_area = Rect::new(
                    list_area.left(),
                    list_area.top(),
                    list_area.width,
                    required_height,
                );
                match self.tile_type {
                    TileType::Bordered => {
                        let border_type = match self.tile_border_type {
                            TileBorderType::Rounded => BorderType::Rounded,
                            TileBorderType::Sharp => BorderType::Plain,
                        };
                        let block = Block::default()
                            .borders(Borders::ALL)
                            .border_style(self.highlight_style)
                            .border_type(border_type);
                        let inner = block.inner(tile_area);
                        render_content(item, inner, buf);
                        block.render(tile_area, buf);
                    }
                    TileType::Unbordered => {
                        render_content(item, tile_area, buf);
                    }
                }
                buf.set_style(tile_area, self.highlight_style);
            }
        }
    }
}

impl TileList<'_> {
    fn get_tile_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_width: usize,
        extra_width: usize,
    ) -> (usize, usize) {
        let offset = offset.min(self.items.len().saturating_sub(1));
        let mut first = offset;
        let mut last = offset;
        let mut width = 0usize;

        for item in self.items.iter().skip(offset) {
            if width + item.width() + extra_width > max_width {
                break;
            }
            width += item.width() + extra_width + 1; // +1 for gap between tiles
            last += 1;
        }

        let index_to_display = selected.unwrap_or(offset);

        while index_to_display >= last {
            width = width.saturating_add(self.items[last].width() + extra_width);
            last += 1;
            while width > max_width {
                width = width.saturating_sub(self.items[first].width() + extra_width);
                first += 1;
            }
        }

        while index_to_display < first {
            first -= 1;
            width = width.saturating_add(self.items[first].width() + extra_width);
            while width > max_width {
                last -= 1;
                width = width.saturating_sub(self.items[last].width() + extra_width);
            }
        }

        (first, last)
    }
}
