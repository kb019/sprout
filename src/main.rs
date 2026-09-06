/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod view;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

/// Color palette.
pub mod palette;

/// App Logo Progress Displayer
pub mod sprout;

/// Vendor widgets.
pub mod vendor;

/// Widgets.
pub mod widgets;

///used to store all the states of the application
pub mod state;

pub mod symbols;

pub mod utils;

pub mod constants;

pub mod model;

pub mod controller;

pub mod samples;

use std::io::{Write, stderr, stdout};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use app::App;
use clap::{Parser, Subcommand, builder::styling};
use constants::TICK_RATE_MS;
use event::{AppEvent, EventHandler};
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;

use crate::controller::Actions;
use crate::model::habit::HabitDb;
use crate::model::settings::SettingsDb;
use crate::state::States;
use crate::update::{
    handle, handle_active_days_event, handle_add_habit_event, handle_best_streaks_event,
    handle_daily_progress_event, handle_delete_habit_event, handle_edit_habit_event,
    handle_get_streak_event, handle_heatmap_data_event, handle_heatmap_year_habits_event,
    handle_log_habit_event, handle_monthly_progress_event, handle_reset_event,
    handle_settings_save_event, handle_weekly_average_event, handle_weekly_progress_event,
    handle_yearly_progress_event,
};
use crate::widgets::notifier::Notifier;

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(version, about = "Track your habit activity in the terminal", styles = STYLES)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(
        about = "Log a value for a habit (e.g. `habit log reading 30`)",
        next_help_heading = "Arguments"
    )]
    Log {
        #[arg(
            help = "Name of the habit to log, e.g. reading, pushups, meditation (must exist — run `habit add` first)"
        )]
        habit_type: String,

        #[arg(
            default_value_t = 1,
            help = "Amount to record in the habit's unit, e.g. 30 for 30 minutes or 15 for 15 pages (default: 1)"
        )]
        value: u32,
    },

    #[command(
        about = "Add a new habit to track (e.g. `habit add --name reading --unit pages --goal 50`)",
        next_help_heading = "Arguments"
    )]
    Add {
        #[arg(
            short,
            long,
            help = "Unique name for the habit, e.g. reading, meditation, pushups"
        )]
        name: String,

        #[arg(
            short,
            long,
            help = "Unit of measurement for logged values, e.g. pages, reps, minutes, hours"
        )]
        unit: String,

        #[arg(
            short,
            long,
            help = "Daily target in the chosen unit, e.g. 30 for 30 minutes or 50 for 50 pages (used to shade the heatmap)"
        )]
        goal: Option<u32>,
    },

    #[command(about = "List all tracked habits with their unit and daily goal")]
    List {},

    #[command(about = "Populate the database with 4 sample habits and 90 days of history")]
    Sample {},
}

///referred from https://github.com/ClementTsang/bottom/blob/main/src/lib.rs#L93
/// Check and report to the user if the current environment is not a terminal.
fn check_if_terminal() {
    use crossterm::tty::IsTty;

    if !stdout().is_tty() {
        eprintln!(
            "Warning: Sprout is not being output to a terminal. Things might not work properly."
        );
        eprintln!("If you're stuck, press 'q', 'Q', or 'Ctrl-c' to quit the program.");
        stderr().flush().expect("should succeed in flushing stderr");
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let db_path = exe_dir.join("habit.db");

    let is_sample = matches!(args.command, Some(Commands::Sample {}));

    check_if_terminal();
    let loaded_settings = {
        let settings_db = SettingsDb::new(&db_path)?;
        settings_db.load_settings().unwrap_or_default()
    };

    // Sample mode writes to a temp file so all controller connections share the same DB
    // without touching habit.db. Cleaned up on exit and on panic (tui.rs hook).
    let actions_db_path = if is_sample {
        let p = std::env::temp_dir().join("sprout_sample.db");
        let _ = std::fs::remove_file(&p);
        p
    } else {
        db_path.clone()
    };

    // Load existing habits and today's completion state once at startup.
    let (
        initial_habits,
        initial_completed,
        initial_streaks,
        initial_daily,
        initial_weekly,
        initial_monthly,
        initial_yearly,
        initial_best_streaks,
        initial_active_days,
        initial_weekly_completion,
        initial_heatmap_year_habits,
    ) = {
        let habit_db = HabitDb::new(&actions_db_path)?;
        if is_sample {
            habit_db
                .execute_batch(samples::SEED_SQL)
                .context("Failed to seed sample data")?;
        }
        let habits = habit_db.get_all_habits()?;
        let completed = habit_db.get_completed_habit_ids_today()?;
        let mut streaks = std::collections::HashMap::new();
        for h in &habits {
            streaks.insert(h.id, habit_db.get_streak(h.id)?);
        }
        let daily = habit_db.get_daily_progress()?;
        let weekly = habit_db.get_weekly_progress()?;
        let monthly = habit_db.get_monthly_progress()?;
        let yearly = habit_db.get_yearly_progress()?;
        let best_streaks = habit_db.get_best_streaks()?;
        let active_days = habit_db.get_active_days_count()?;
        let weekly_completion = habit_db.get_weekly_completion_by_day()?;
        let heatmap_year_habits = habit_db.get_heatmap_year_habits()?;
        (
            habits,
            completed,
            streaks,
            daily,
            weekly,
            monthly,
            yearly,
            best_streaks,
            active_days,
            weekly_completion,
            heatmap_year_habits,
        )
    };

    let mut app = App::new();
    app.habits = initial_habits;
    app.completed_habits = initial_completed.into_iter().collect();
    app.active_streaks = initial_streaks;
    app.daily_progress = initial_daily;
    app.weekly_progress = initial_weekly;
    app.monthly_progress = initial_monthly;
    app.yearly_progress = initial_yearly;
    app.best_streaks = initial_best_streaks;
    app.active_days = initial_active_days;
    app.weekly_completion = initial_weekly_completion;
    app.heatmap_year_habits = initial_heatmap_year_habits;

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_MS);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let actions = Actions::new(tui.events.sender.clone(), actions_db_path.clone());
    let mut states = States::new();
    states
        .app_state
        .update_heatmap_tile_states(&app.heatmap_year_habits);

    let (_, tile_states) = states.app_state.settings_states_mut();
    if let Some(v) = loaded_settings
        .get("theme")
        .and_then(|s| s.parse::<usize>().ok())
    {
        app.active_theme = v;
        if let Some(s) = tile_states.get_mut(0) {
            s.select(Some(v));
        }
    }
    if let Some(v) = loaded_settings
        .get("default_view")
        .and_then(|s| s.parse::<usize>().ok())
        && let Some(s) = tile_states.get_mut(1)
    {
        s.select(Some(v));
    }
    if let Some(v) = loaded_settings
        .get("cursor_blink")
        .and_then(|s| s.parse::<usize>().ok())
    {
        app.cursor_blink_enabled = v == 0;
        if let Some(s) = tile_states.get_mut(2) {
            s.select(Some(v));
        }
    }
    if let Some(v) = loaded_settings
        .get("notification_level")
        .and_then(|s| s.parse::<usize>().ok())
    {
        app.notification_level = v;
        if let Some(s) = tile_states.get_mut(3) {
            s.select(Some(v));
        }
    }
    let mut notifier = Notifier::new();

    if let Some((year, ids)) = app.heatmap_year_habits.first()
        && let Some(&habit_id) = ids.first()
    {
        actions.fetch_heatmap_data(habit_id, *year);
    }

    while !app.should_quit {
        tui.draw(&mut app, &mut states, &mut notifier)?;
        match tui.events.next()? {
            AppEvent::Key(key_event) => {
                handle(&mut app, key_event, &mut states, &actions, &mut notifier);
            }
            AppEvent::Tick => {
                app.tick();
                notifier.tick();
                if app.best_streak_refresh_delay > 0 {
                    app.best_streak_refresh_delay -= 1;
                    if app.best_streak_refresh_delay == 0 {
                        actions.fetch_best_streaks();
                    }
                }
                if app.active_days_refresh_delay > 0 {
                    app.active_days_refresh_delay -= 1;
                    if app.active_days_refresh_delay == 0 {
                        actions.fetch_active_days();
                    }
                }
                if app.weekly_average_refresh_delay > 0 {
                    app.weekly_average_refresh_delay -= 1;
                    if app.weekly_average_refresh_delay == 0 {
                        actions.fetch_weekly_average();
                    }
                }
                if app.heatmap_year_habits_refresh_delay > 0 {
                    app.heatmap_year_habits_refresh_delay -= 1;
                    if app.heatmap_year_habits_refresh_delay == 0 {
                        actions.fetch_heatmap_year_habits();
                    }
                }
                if app.settings_save_delay > 0 {
                    app.settings_save_delay -= 1;
                    if app.settings_save_delay == 0 {
                        actions.save_settings(
                            states.app_state.active_theme(),
                            states.app_state.settings_tile_selected(1).unwrap_or(0),
                            states.app_state.settings_tile_selected(2).unwrap_or(0),
                            states.app_state.settings_tile_selected(3).unwrap_or(0),
                        );
                    }
                }
            }
            AppEvent::Mouse(_) | AppEvent::Resize(_, _) => {}
            AppEvent::AddHabit(event) => {
                handle_add_habit_event(&mut app, event, &mut states, &mut notifier, &actions);
            }
            AppEvent::LogHabit(event) => {
                handle_log_habit_event(&mut app, event, &mut states, &mut notifier, &actions);
            }
            AppEvent::DeleteHabit(event) => {
                handle_delete_habit_event(&mut app, event, &mut states, &mut notifier, &actions);
            }
            AppEvent::GetStreak(event) => {
                handle_get_streak_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::DailyProgress(event) => {
                handle_daily_progress_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::WeeklyProgress(event) => {
                handle_weekly_progress_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::MonthlyProgress(event) => {
                handle_monthly_progress_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::YearlyProgress(event) => {
                handle_yearly_progress_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::EditHabit(event) => {
                handle_edit_habit_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::BestStreaks(event) => {
                handle_best_streaks_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::ActiveDays(event) => {
                handle_active_days_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::WeeklyAverage(event) => {
                handle_weekly_average_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::HeatmapYearHabits(event) => {
                handle_heatmap_year_habits_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::HeatmapData(event) => {
                handle_heatmap_data_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::Reset(event) => {
                handle_reset_event(&mut app, event, &mut states, &mut notifier);
            }
            AppEvent::SaveSettings(event) => {
                handle_settings_save_event(event, &mut states, &mut notifier);
            }
        }
    }

    tui.exit()?;
    if is_sample {
        let _ = std::fs::remove_file(&actions_db_path);
    }
    Ok(())
}
