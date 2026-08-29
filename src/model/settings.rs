use std::path::Path;

use anyhow::Result;
use rusqlite::Connection;

use crate::model::open_db;

pub struct SettingsDb {
    conn: Connection,
}

impl SettingsDb {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = open_db(path)?;
        let settings_db = Self { conn };
        settings_db.create_settings_table()?;
        Ok(settings_db)
    }

    pub fn create_settings_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}
