// use anyhow::{Context, Result};
/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

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

use std::io::{Write, stderr, stdout};
use std::thread;
use std::time::Duration;

use app::App;
use clap::{Parser, Subcommand, builder::styling};
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::widgets::ListState;
use ratatui::{Terminal, backend::CrosstermBackend};
use tui::Tui;
use update::update;

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
    check_if_terminal();
    println!("argus: {args:?}");
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut list_state = ListState::default().with_selected(Some(0));
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app, &mut list_state)?;
        // Handle events.
        match tui.events.next()? {
            Event::Key(key_event) => update(&mut app, key_event, &mut list_state),
            Event::Tick | Event::Mouse(_) | Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
