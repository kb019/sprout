use std::{io, panic};

use anyhow::{Result, anyhow};
use ratatui::{
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    },
    style::Style,
    widgets::Block,
};
pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

use crate::{app::App, event::EventHandler, state::States, view, widgets::notifier::Notifier};

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
    /// Interface to the Terminal.
    terminal: CrosstermTerminal,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            let _ = std::fs::remove_file(std::env::temp_dir().join("sprout_sample.db"));
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal
            .hide_cursor()
            .map_err(|e| anyhow!("failed to hide cursor: {e}"))?;
        self.terminal
            .clear()
            .map_err(|e| anyhow!("failed to clear terminal: {e}"))?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::view:render
    pub fn draw(
        &mut self,
        app: &mut App,
        states: &mut States,
        notifier: &mut Notifier,
    ) -> Result<()> {
        notifier.set_palette(app.palette());
        notifier.set_level(app.notification_level);
        self.terminal.draw(|frame| {
            frame.render_widget(
                Block::default().style(Style::default().bg(app.palette().background)),
                frame.area(),
            );
            view::render(app, frame, states);
            if app.is_modal_in_focus() {
                view::modals::render_modals(app, frame, states);
            }
            frame.render_widget(&*notifier, frame.area());
        })?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
