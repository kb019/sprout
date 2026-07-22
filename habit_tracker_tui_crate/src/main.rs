// use anyhow::{Context, Result};
use clap::{Parser, Subcommand, builder::styling};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(version, about = "Track your habit activity in the terminal", styles = STYLES)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
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

fn main() {
    let args = Cli::parse();
    let _ = args;
}
