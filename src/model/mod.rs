use std::{fs::create_dir_all, path::Path};

use anyhow::{Context, Result};
use rusqlite::Connection;

pub mod habit;
pub mod settings;

pub fn open_db(path: &Path) -> Result<Connection> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let conn = Connection::open(path)
        .with_context(|| format!("Failed to open database at: {}", path.display()))?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")
        .context("Failed to enable foreign key enforcement")?;
    Ok(conn)
}
