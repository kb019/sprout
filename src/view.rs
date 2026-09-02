mod dashboard;
mod heatmap_ui;
mod menu;
pub mod modals;
mod settings;
mod stats;

use crate::app::App;
use crate::state::States;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line as TextLine, Span};

pub fn render(app: &mut App, frame: &mut Frame, states: &mut States) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [menu_column, app_column] = main.layout(&horizontal);

    menu::render_menu_column(app, frame, menu_column, states.app_state.menu_state_mut());

    if let Some(selected) = states.app_state.menu_state().selected() {
        match selected {
            0 => dashboard::render_dashboard(app, frame, app_column, states),
            1 => heatmap_ui::render_heatmap_page(
                app,
                frame,
                app_column,
                states.app_state.heatmap_tile_state_mut(),
            ),
            2 => stats::render_stats_column(app, frame, app_column, states),
            3 => {
                let (settings_state, settings_tile_states) = states.app_state.settings_states_mut();
                settings::render_settings_page(
                    app,
                    frame,
                    app_column,
                    settings_state,
                    settings_tile_states,
                );
            }
            _ => {}
        }
    }

    draw_app_name(app, frame, top);
}

fn draw_app_name(app: &App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let title: TextLine<'_> = TextLine::from_iter([
        Span::from("sprout").style(Style::new().fg(p.accent).bold()),
        Span::from(" - habit tracker").style(Style::new().fg(p.fg_dim)),
    ]);
    frame.render_widget(title.left_aligned(), area);
}
