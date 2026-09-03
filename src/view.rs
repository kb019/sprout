mod dashboard;
mod heatmap_ui;
mod menu;
pub mod modals;
mod settings;
mod stats;

use crate::app::App;
use crate::state::States;
use crate::widgets::tile_list::{TileDirection, TileItem, TileList, TileType};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::{Block, Borders, Padding, Widget};

pub fn render(app: &mut App, frame: &mut Frame, states: &mut States) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .spacing(1);
    let horizontal =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).spacing(1);
    let [top, main, help_area] = frame.area().layout(&vertical);
    let [menu_column, app_column] = main.layout(&horizontal);

    let selected_menu = states.app_state.menu_state().selected();

    menu::render_menu_column(app, frame, menu_column, states.app_state.menu_state_mut());

    if let Some(selected) = selected_menu {
        match selected {
            0 => dashboard::render_dashboard(app, frame, app_column, states),
            1 => heatmap_ui::render_heatmap_page(app, frame, app_column, states),
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
    draw_help_bar(app, frame, help_area, selected_menu);
}

fn draw_app_name(app: &App, frame: &mut Frame, area: Rect) {
    let p = app.palette();
    let title: TextLine<'_> = TextLine::from_iter([
        Span::from("sprout").style(Style::new().fg(p.accent).bold()),
        Span::from(" - habit tracker").style(Style::new().fg(p.fg_dim)),
    ]);
    frame.render_widget(title.left_aligned(), area);
}

fn draw_help_bar(app: &App, frame: &mut Frame, area: Rect, selected_menu: Option<usize>) {
    let p = app.palette();

    const UNIVERSAL: &[(&str, &str)] =
        &[("↑↓", "navigate"), ("tab", "switch panel"), ("?", "help")];

    let screen: &[(&str, &str)] = match selected_menu {
        Some(0) => &[
            ("enter", "toggle"),
            ("+", "add"),
            ("e", "edit"),
            ("x", "delete"),
        ],
        Some(1) => &[("◂ ▸", "navigate habits")],
        Some(2) => &[("4", "streak leaderboard")],
        Some(3) => &[("enter", "toggle/activate"), ("◂ ▸", "change option")],
        _ => &[],
    };

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::new().fg(p.border))
        .padding(Padding::horizontal(1));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<TileItem> = UNIVERSAL
        .iter()
        .chain(screen.iter())
        .map(|(key, desc)| {
            TileItem::new(TextLine::from(vec![
                Span::from(*key).style(Style::new().fg(p.fg).bold()),
                Span::from(" ").style(Style::new().fg(p.fg_dim)),
                Span::from(*desc).style(Style::new().fg(p.fg_dim)),
            ]))
        })
        .collect();

    let tile_list = TileList::new(items)
        .tile_type(TileType::Unbordered)
        .direction(TileDirection::LeftToRight);

    Widget::render(tile_list, inner, frame.buffer_mut());
}
