use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::event::{AppEvent, SettingsSaveEvent};
use crate::model::settings::SettingsDb;

pub struct SaveSettingsAction {
    sender: mpsc::Sender<AppEvent>,
    db_path: PathBuf,
}

impl SaveSettingsAction {
    pub fn new(sender: mpsc::Sender<AppEvent>, db_path: PathBuf) -> Self {
        Self { sender, db_path }
    }

    pub fn save(&self, theme: usize, cursor_blink: usize, notification_level: usize) {
        let sender = self.sender.clone();
        let db_path = self.db_path.clone();
        let _ = sender.send(AppEvent::SaveSettings(SettingsSaveEvent::Saving));
        thread::spawn(move || {
            let result = SettingsDb::new(&db_path).and_then(|db| {
                db.save_setting("theme", &theme.to_string())?;
                db.save_setting("cursor_blink", &cursor_blink.to_string())?;
                db.save_setting("notification_level", &notification_level.to_string())?;
                Ok(())
            });
            match result {
                Ok(()) => {
                    let _ = sender.send(AppEvent::SaveSettings(SettingsSaveEvent::Saved));
                }
                Err(e) => {
                    let _ = sender.send(AppEvent::SaveSettings(SettingsSaveEvent::Failed(
                        e.to_string(),
                    )));
                }
            }
        });
    }
}
