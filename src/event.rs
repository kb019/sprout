use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
mod app_event;
pub use app_event::{
    ActiveDaysEvent, AddHabitEvent, BestStreaksEvent, DailyProgressEvent, DeleteHabitEvent,
    EditHabitEvent, GetStreakEvent, HeatmapDataEvent, HeatmapYearHabitsEvent, LogHabitEvent,
    MonthlyProgressEvent, ResetEvent, SettingsSaveEvent, WeeklyAverageEvent, WeeklyProgressEvent,
    YearlyProgressEvent,
};

/// Terminal events.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum AppEvent {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),

    AddHabit(AddHabitEvent),
    EditHabit(EditHabitEvent),
    LogHabit(LogHabitEvent),
    DeleteHabit(DeleteHabitEvent),
    GetStreak(GetStreakEvent),
    DailyProgress(DailyProgressEvent),
    WeeklyProgress(WeeklyProgressEvent),
    MonthlyProgress(MonthlyProgressEvent),
    YearlyProgress(YearlyProgressEvent),
    BestStreaks(BestStreaksEvent),
    ActiveDays(ActiveDaysEvent),
    WeeklyAverage(WeeklyAverageEvent),
    HeatmapYearHabits(HeatmapYearHabitsEvent),
    HeatmapData(HeatmapDataEvent),
    Reset(ResetEvent),
    SaveSettings(SettingsSaveEvent),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    #[allow(dead_code)]
    pub sender: mpsc::Sender<AppEvent>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<AppEvent>,
    /// Event handler thread.
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(AppEvent::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(AppEvent::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(AppEvent::Resize(w, h)),
                            _ => Ok(()),
                        }
                        .expect("failed to send terminal event");
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(AppEvent::Tick)
                            .expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<AppEvent> {
        Ok(self.receiver.recv()?)
    }
}
