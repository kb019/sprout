mod dashboard;
mod heatmap_ui;
mod menu;
mod settings;
mod stats;
use crate::app::App;
use crate::palette::Palette;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::ListState;

pub fn render(
    app: &mut App,
    frame: &mut Frame,
    list_state: &mut ListState,
    tile_state: &mut ListState,
) {
    let vertical: Layout =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [menu_column, app_column] = main.layout(&horizontal);

    menu::render_menu_column(app, frame, menu_column, list_state);
    if let Some(current_menu_selected) = list_state.selected() {
        if current_menu_selected == 0 {
            dashboard::render_dashboard(app, frame, app_column);
        } else if current_menu_selected == 2 {
            stats::render_stats_column(app, frame, app_column);
        } else if current_menu_selected == 1 {
            heatmap_ui::render_heatmap_page(app, frame, app_column, tile_state);
        } else if current_menu_selected == 3 {
            settings::render_settings_page(app, frame, app_column);
        }
    };
    draw_app_name(frame, top);
}

fn draw_app_name(frame: &mut Frame, area: Rect) {
    let title: TextLine<'_> = TextLine::from_iter([
        Span::from("sprout").style(Style::new().fg(Palette::BRAND_GREEN).bold()),
        Span::from(" - habit tracker").style(Style::new().fg(Palette::TEXT_SECONDARY)),
    ]);
    frame.render_widget(title.left_aligned(), area);
}
