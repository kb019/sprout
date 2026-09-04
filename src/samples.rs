use anyhow::{Context, Result};
use std::path::Path;

use crate::model::open_db;

pub const SEED_SQL: &str = include_str!("../samples/seed.sql");

pub fn populate(db_path: &Path) -> Result<()> {
    let conn = open_db(db_path)?;

    let existing: i64 = conn
        .query_row("SELECT COUNT(*) FROM habit", [], |row| row.get(0))
        .unwrap_or(0);

    if existing > 0 {
        eprintln!(
            "Database already contains {} habit(s). Remove {} first to load sample data.",
            existing,
            db_path.display()
        );
        std::process::exit(1);
    }

    conn.execute_batch(SEED_SQL)
        .context("Failed to execute sample seed SQL")?;

    Ok(())
}
