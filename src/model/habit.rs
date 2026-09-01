use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::Local;
use rusqlite::{Connection, Error, params};

use crate::model::open_db;

#[derive(Debug, Clone)]
pub struct HabitLog {
    pub id: i32,
    pub habit_id: i32,
    pub date: String,
    pub completed: bool,
    pub progress: i32,
}

#[derive(Debug, Clone)]
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub daily_goal: i32,
    pub weekly_goal: i32,
    pub monthly_goal: i32,
    pub yearly_goal: i32,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub struct NewHabit {
    pub name: String,
    pub daily_goal: i32,
    pub weekly_goal: i32,
    pub monthly_goal: i32,
    pub yearly_goal: i32,
}

pub struct HabitUpdate {
    pub id: i32,
    pub name: String,
    pub daily_goal: i32,
    pub weekly_goal: i32,
    pub monthly_goal: i32,
    pub yearly_goal: i32,
}

pub struct HabitDb {
    conn: Connection,
}

impl HabitDb {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = open_db(path)?;
        let habit_db = Self { conn };
        habit_db
            .create_habit_table()
            .with_context(|| format!("Failed to create habit table at {}", path.display()))?;
        habit_db
            .create_habit_log_table()
            .with_context(|| format!("Failed to create habit_log table at {}", path.display()))?;
        Ok(habit_db)
    }

    pub fn create_habit(&mut self, habit: &NewHabit) -> Result<Habit> {
        let tx = self.conn.transaction().with_context(|| {
            format!(
                "Failed to begin transaction for create_habit '{}'",
                habit.name
            )
        })?;

        tx.execute(
            "INSERT INTO habit (habit_name, streaks, daily_goal, weekly_goal, monthly_goal, yearly_goal) \
             VALUES (?1, 0, ?2, ?3, ?4, ?5)",
            params![habit.name, habit.daily_goal, habit.weekly_goal, habit.monthly_goal, habit.yearly_goal],
        )
        .with_context(|| format!("Failed to insert habit '{}'", habit.name))?;

        let id = tx.last_insert_rowid() as i32;

        let new_habit = tx
            .query_row(
                "SELECT id, habit_name, daily_goal, weekly_goal, monthly_goal, yearly_goal, created_at \
                 FROM habit WHERE id = ?1",
                params![id],
                |row| Ok(Habit {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    daily_goal: row.get(2)?,
                    weekly_goal: row.get(3)?,
                    monthly_goal: row.get(4)?,
                    yearly_goal: row.get(5)?,
                    created_at: row.get(6)?,
                }),
            )
            .with_context(|| format!("Failed to fetch created habit with id={}", id))?;

        tx.commit().with_context(|| {
            format!(
                "Failed to commit create_habit transaction for '{}'",
                habit.name
            )
        })?;

        Ok(new_habit)
    }

    pub fn get_all_habits(&self) -> Result<Vec<Habit>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, habit_name, daily_goal, weekly_goal, monthly_goal, yearly_goal, created_at \
                 FROM habit",
            )
            .context("Failed to prepare get_all_habits statement")?;

        let habits = stmt
            .query_map([], |row| {
                Ok(Habit {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    daily_goal: row.get(2)?,
                    weekly_goal: row.get(3)?,
                    monthly_goal: row.get(4)?,
                    yearly_goal: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })
            .context("Failed to query habits")?
            .collect::<std::result::Result<Vec<_>, _>>()
            .context("Failed to collect habits")?;

        Ok(habits)
    }

    pub fn get_completed_habit_ids_today(&self) -> Result<Vec<i32>> {
        let mut stmt = self
            .conn
            .prepare("SELECT habit_id FROM habit_log WHERE date = date('now', 'localtime') AND completed = 1")
            .context("Failed to prepare get_completed_habit_ids_today statement")?;

        let ids = stmt
            .query_map([], |row| row.get::<_, i32>(0))
            .context("Failed to query completed habit ids")?
            .collect::<std::result::Result<Vec<_>, _>>()
            .context("Failed to collect completed habit ids")?;

        Ok(ids)
    }

    pub fn set_completion(&self, habit_id: i32, completed: bool) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO habit_log (habit_id, date, completed)
                 VALUES (?1, date('now', 'localtime'), ?2)
                 ON CONFLICT(habit_id, date) DO UPDATE SET completed = excluded.completed",
                params![habit_id, completed as i32],
            )
            .with_context(|| {
                format!(
                    "Failed to set completion for habit_id={} completed={}",
                    habit_id, completed
                )
            })?;
        Ok(())
    }

    pub fn is_completed_today(&self, habit_id: i32) -> Result<bool> {
        let result = self.conn.query_row(
            "SELECT completed FROM habit_log WHERE habit_id = ?1 AND date = date('now', 'localtime')",
            params![habit_id],
            |row| row.get::<_, i32>(0),
        );
        match result {
            Ok(v) => Ok(v != 0),
            Err(Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(e)
                .with_context(|| format!("Failed to check completion for habit_id={}", habit_id)),
        }
    }

    pub fn get_streak(&self, habit_id: i32) -> Result<i32> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT date FROM habit_log
                 WHERE habit_id = ?1 AND completed = 1
                 ORDER BY date DESC",
            )
            .with_context(|| {
                format!(
                    "Failed to prepare get_streak statement for habit_id={}",
                    habit_id
                )
            })?;

        let dates: Vec<String> = stmt
            .query_map(params![habit_id], |row| row.get(0))
            .with_context(|| format!("Failed to query streak dates for habit_id={}", habit_id))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .with_context(|| format!("Failed to collect streak dates for habit_id={}", habit_id))?;

        let mut streak = 0i32;
        let mut expected = Local::now().date_naive();
        for date_str in dates {
            let Ok(d) = date_str.parse::<chrono::NaiveDate>() else {
                break;
            };
            if d == expected {
                streak += 1;
                expected = expected.pred_opt().unwrap_or(expected);
            } else {
                break;
            }
        }
        Ok(streak)
    }

    pub fn log_habit_progress(
        &mut self,
        habit_id: i32,
        completed: i32,
        progress: i32,
    ) -> Result<HabitLog> {
        let tx = self.conn.transaction().with_context(|| {
            format!(
                "Failed to begin transaction for log_habit_progress habit_id={}",
                habit_id
            )
        })?;

        tx.execute(
            "INSERT INTO habit_log (habit_id, date, completed, progress)
             VALUES (?1, date('now', 'localtime'), ?2, ?3)
             ON CONFLICT(habit_id, date) DO UPDATE SET
                 completed = excluded.completed,
                 progress  = excluded.progress",
            params![habit_id, completed, progress],
        )
        .with_context(|| format!("Failed to upsert habit_log for habit_id={}", habit_id))?;

        let log = tx
            .query_row(
                "SELECT id, habit_id, date, completed, progress FROM habit_log \
                 WHERE habit_id = ?1 AND date = date('now', 'localtime')",
                params![habit_id],
                |row| {
                    Ok(HabitLog {
                        id: row.get(0)?,
                        habit_id: row.get(1)?,
                        date: row.get(2)?,
                        completed: row.get::<_, i32>(3)? != 0,
                        progress: row.get(4)?,
                    })
                },
            )
            .with_context(|| {
                format!(
                    "Failed to fetch habit_log after upsert for habit_id={}",
                    habit_id
                )
            })?;

        tx.commit().with_context(|| {
            format!(
                "Failed to commit log_habit_progress for habit_id={}",
                habit_id
            )
        })?;

        Ok(log)
    }

    pub fn update_habit(&mut self, update: &HabitUpdate, created_at: String) -> Result<Habit> {
        let tx = self.conn.transaction().with_context(|| {
            format!(
                "Failed to begin transaction for update_habit id={}",
                update.id
            )
        })?;
        tx.execute(
            "UPDATE habit SET habit_name=?1, daily_goal=?2, weekly_goal=?3, monthly_goal=?4, yearly_goal=?5 WHERE id=?6",
            params![update.name, update.daily_goal, update.weekly_goal, update.monthly_goal, update.yearly_goal, update.id],
        )
        .with_context(|| format!("Failed to update habit id={}", update.id))?;
        tx.commit().with_context(|| {
            format!(
                "Failed to commit update_habit transaction for id={}",
                update.id
            )
        })?;
        Ok(Habit {
            id: update.id,
            name: update.name.clone(),
            daily_goal: update.daily_goal,
            weekly_goal: update.weekly_goal,
            monthly_goal: update.monthly_goal,
            yearly_goal: update.yearly_goal,
            created_at,
        })
    }

    pub fn delete_habit(&mut self, habit_id: i32) -> Result<()> {
        let tx = self.conn.transaction().with_context(|| {
            format!(
                "Failed to begin transaction for delete_habit id={}",
                habit_id
            )
        })?;
        tx.execute("DELETE FROM habit WHERE id = ?1", params![habit_id])
            .with_context(|| format!("Failed to delete habit id={}", habit_id))?;
        tx.commit().with_context(|| {
            format!(
                "Failed to commit delete_habit transaction for id={}",
                habit_id
            )
        })?;
        Ok(())
    }

    fn query_progress_in_period(&self, sql: &str) -> Result<HashMap<i32, i32>> {
        let mut stmt = self
            .conn
            .prepare(sql)
            .context("Failed to prepare progress query")?;
        let map = stmt
            .query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?)))
            .context("Failed to query progress")?
            .collect::<std::result::Result<HashMap<_, _>, _>>()
            .context("Failed to collect progress")?;
        Ok(map)
    }

    pub fn get_daily_progress(&self) -> Result<HashMap<i32, i32>> {
        self.query_progress_in_period(
            "SELECT habit_id, COALESCE(SUM(progress), 0) FROM habit_log \
             WHERE date = date('now', 'localtime') GROUP BY habit_id",
        )
        .context("Failed to get daily progress")
    }

    pub fn get_weekly_progress(&self) -> Result<HashMap<i32, i32>> {
        self.query_progress_in_period(
            "SELECT habit_id, COALESCE(SUM(progress), 0) FROM habit_log \
             WHERE strftime('%Y-%W', date) = strftime('%Y-%W', 'now', 'localtime') GROUP BY habit_id",
        )
        .context("Failed to get weekly progress")
    }

    pub fn get_monthly_progress(&self) -> Result<HashMap<i32, i32>> {
        self.query_progress_in_period(
            "SELECT habit_id, COALESCE(SUM(progress), 0) FROM habit_log \
             WHERE strftime('%Y-%m', date) = strftime('%Y-%m', 'now', 'localtime') GROUP BY habit_id",
        )
        .context("Failed to get monthly progress")
    }

    pub fn get_yearly_progress(&self) -> Result<HashMap<i32, i32>> {
        self.query_progress_in_period(
            "SELECT habit_id, COALESCE(SUM(progress), 0) FROM habit_log \
             WHERE strftime('%Y', date) = strftime('%Y', 'now', 'localtime') GROUP BY habit_id",
        )
        .context("Failed to get yearly progress")
    }

    fn create_habit_table(&self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS habit (
                    id           INTEGER PRIMARY KEY,
                    habit_name   TEXT    NOT NULL,
                    streaks      INTEGER NOT NULL DEFAULT 0,
                    daily_goal   INTEGER NOT NULL DEFAULT 0,
                    weekly_goal  INTEGER NOT NULL DEFAULT 0,
                    monthly_goal INTEGER NOT NULL DEFAULT 0,
                    yearly_goal  INTEGER NOT NULL DEFAULT 0,
                    created_at   TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
                )",
                [],
            )
            .context("Failed to execute CREATE TABLE habit")?;
        Ok(())
    }

    fn create_habit_log_table(&self) -> Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS habit_log (
                    id        INTEGER PRIMARY KEY,
                    habit_id  INTEGER NOT NULL REFERENCES habit(id) ON DELETE CASCADE,
                    date      TEXT    NOT NULL,
                    completed INTEGER NOT NULL DEFAULT 0,
                    progress  INTEGER NOT NULL DEFAULT 0,
                    UNIQUE(habit_id, date)
                )",
                [],
            )
            .context("Failed to execute CREATE TABLE habit_log")?;
        Ok(())
    }
}
