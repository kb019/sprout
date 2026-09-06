use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::{Connection, params};

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

    pub fn save_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
                params![key, value],
            )
            .context("Failed to save setting")?;
        Ok(())
    }

    pub fn load_settings(&self) -> Result<HashMap<String, String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT key, value FROM settings")
            .context("Failed to prepare load settings query")?;
        let map = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .context("Failed to query settings")?
            .filter_map(|r| r.ok())
            .collect();
        Ok(map)
    }
}
