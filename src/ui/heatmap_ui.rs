use crate::app::App;
use crate::palette::Palette;
use crate::widgets::heatmap::HeatMapGen;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, ListState, Padding, Widget};

pub fn render_heatmap_page(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    tile_state: &mut ListState,
) {
    let heat_map_block = Block::default()
        .title(" activity heatmap ")
        .borders(Borders::ALL)
        .border_style(Style::new().fg(crate::palette::Palette::BRAND_GREEN))
        .padding(Padding::new(1, 1, 1, 1));
    let block_inner_area = heat_map_block.inner(area);
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
    let [habit_tiles_area, heatmap_area, legend_area] = block_inner_area.layout(&vertical_layout);
    frame.render_widget(heat_map_block, area);
    render_heatmap(frame, heatmap_area);
    render_habits(app, frame, habit_tiles_area, tile_state);
    render_lengend(frame, legend_area);
}

pub fn render_habits(app: &mut App, frame: &mut Frame, area: Rect, tile_state: &mut ListState) {
    let vertical_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(15), Constraint::Max(15)])
        .spacing(2);
    let active_line = Line::from_iter([
        Span::from("176").style(Style::new().fg(Palette::BRAND_GREEN)),
        Span::from(" active days").style(Style::new().fg(Palette::TEXT_SECONDARY)),
    ]);
    let [habit_tiles_area, active_days_area] = area.layout(&vertical_layout);
    frame.render_widget(active_line, active_days_area);
    render_habit_tiles(app, frame, habit_tiles_area, tile_state);
}

pub fn render_heatmap(frame: &mut Frame, area: Rect) {
    frame.render_widget(HeatMapGen::new(), area);
}

pub fn render_lengend(frame: &mut Frame, area: Rect) {
    let legend_line = Line::from_iter([
        Span::from("less").style(Style::new().fg(Palette::TEXT_SECONDARY)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_0)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_1)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_2)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_3)),
        Span::from(" "),
        Span::from("  ").style(Style::new().bg(Palette::HEATMAP_4)),
        Span::from(" "),
        Span::from("more").style(Style::new().fg(Palette::TEXT_SECONDARY)),
    ]);
    frame.render_widget(legend_line, area);
}

// Renders the habit tiles at the top of the heatmap page
// logic referred from raratui list rendering logic
#[allow(clippy::needless_pass_by_ref_mut)]
pub fn render_habit_tiles(
    app: &mut App,
    frame: &mut Frame,
    area: Rect,
    tile_state: &mut ListState,
) {
    let tiles: Vec<Text> = app
        .habits
        .iter()
        .enumerate()
        .map(|(i, menu_item)| {
            let mut bold_modifier = Modifier::empty();
            if let Some(select) = tile_state.selected()
                && select == i
            {
                bold_modifier = Modifier::BOLD;
            }
            Text::from(menu_item.clone())
                .add_modifier(bold_modifier)
                .centered()
        })
        .collect();

    let list_style = Style::new().fg(Palette::TEXT_SECONDARY);
    let highlight_style = Style::new().fg(Palette::BRAND_GREEN);

    let buf = frame.buffer_mut();
    buf.set_style(area, list_style);

    if area.is_empty() || area.height < 3 || tiles.is_empty() {
        *tile_state.offset_mut() = 0;
        return;
    }

    if tile_state.selected().is_some_and(|s| s >= tiles.len()) {
        tile_state.select(Some(tiles.len().saturating_sub(1)));
    }

    let (first_visible, last_visible) = get_tile_bounds(
        &tiles,
        tile_state.selected(),
        tile_state.offset(),
        area.width as usize,
    );

    let mut current_width: u16 = 0;
    for (i, tile) in tiles
        .iter()
        .enumerate()
        .skip(first_visible)
        .take(last_visible - first_visible)
    {
        let tile_width = tile.width() as u16 + 4; // +2 borders + +2 inner padding
        let row_area = Rect::new(area.left() + current_width, area.top(), tile_width, 3);
        current_width += tile_width + 1; // +1 gap between tiles

        let is_selected: bool = tile_state.selected() == Some(i);
        let border_style = if is_selected {
            highlight_style
        } else {
            list_style
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .border_type(BorderType::Rounded);
        let inner = block.inner(row_area);
        Widget::render(tile, inner, buf);
        block.render(row_area, buf);
        if is_selected {
            buf.set_style(row_area, highlight_style);
        }
    }

    let render_truncated_element = last_visible - first_visible == 0;

    if render_truncated_element {
        let selected_tile = tiles.get(tile_state.selected().unwrap_or(0));
        if let Some(tile) = selected_tile {
            let tile_area = Rect::new(area.left(), area.top(), area.width, 3);
            let tile_border_style = highlight_style;
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(tile_border_style)
                .border_type(BorderType::Rounded);
            let inner = block.inner(tile_area);
            Widget::render(tile, inner, buf);
            block.render(tile_area, buf);
            buf.set_style(tile_area, highlight_style);
        }
    }

    // Reset offset so items reflow correctly on resize
    *tile_state.offset_mut() = 0;
}

fn get_tile_bounds(
    tiles: &[Text],
    selected: Option<usize>,
    offset: usize,
    max_width: usize,
) -> (usize, usize) {
    let offset = offset.min(tiles.len().saturating_sub(1));
    let mut first = offset;
    let mut last = offset;
    let mut width = 0usize;

    for tile in tiles.iter().skip(offset) {
        if width + tile.width() + 4 > max_width {
            break;
        }
        width += tile.width() + 4;
        last += 1;
    }

    let index_to_display = selected.unwrap_or(offset);

    while index_to_display >= last {
        width = width.saturating_add(tiles[last].width() + 4);
        last += 1;
        while width > max_width {
            width = width.saturating_sub(tiles[first].width() + 4);
            first += 1;
        }
    }

    while index_to_display < first {
        first -= 1;
        width = width.saturating_add(tiles[first].width() + 4);
        while width > max_width {
            last -= 1;
            width = width.saturating_sub(tiles[last].width() + 4);
        }
    }

    (first, last)
}
