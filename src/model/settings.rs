use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::model::open_db;

pub struct SettingsDb {
    conn: Connection,
}

impl SettingsDb {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = open_db(path)?;
        let settings_db = Self { conn };
        settings_db
            .create_settings_table()
            .with_context(|| format!("Failed to create settings table at {}", path.display()))?;
        Ok(settings_db)
    }

    fn create_settings_table(&self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS settings (
                    id    INTEGER PRIMARY KEY AUTOINCREMENT,
                    key   TEXT    NOT NULL UNIQUE,
                    value TEXT    NOT NULL
                )",
                [],
            )
            .context("Failed to execute CREATE TABLE settings")?;
        Ok(())
    }
}
