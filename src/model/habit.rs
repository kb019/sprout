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

pub struct HabitDb {
    conn: Connection,
}

impl HabitDb {
    pub fn new(path: &Path) -> Result<Self> {
        let conn = open_db(path)?;
        let habit_db = Self { conn };
        habit_db.create_habit_table().with_context(|| {
            format!("Failed to create Habit Table for path: {}", path.display())
        })?;
        habit_db.create_habit_log_table().with_context(|| {
            format!(
                "Failed to create Habit Log Table for path: {}",
                path.display()
            )
        })?;
        Ok(habit_db)
    }

    pub fn create_habit(&self, habit: &NewHabit) -> Result<Habit, Error> {
        self.conn.execute(
            "INSERT INTO habit (habit_name, streaks, daily_goal, weekly_goal, monthly_goal, yearly_goal) \
             VALUES (?1, 0, ?2, ?3, ?4, ?5)",
            params![
                habit.name,
                habit.daily_goal,
                habit.weekly_goal,
                habit.monthly_goal,
                habit.yearly_goal
            ],
        )?;
        let id = self.conn.last_insert_rowid() as i32;
        self.conn.query_row(
            "SELECT id, habit_name, daily_goal, weekly_goal, monthly_goal, yearly_goal, created_at \
             FROM habit WHERE id = ?1",
            params![id],
            |row| {
                Ok(Habit {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    daily_goal: row.get(2)?,
                    weekly_goal: row.get(3)?,
                    monthly_goal: row.get(4)?,
                    yearly_goal: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
    }

    pub fn get_all_habits(&self) -> Result<Vec<Habit>, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, habit_name, daily_goal, weekly_goal, monthly_goal, yearly_goal, created_at FROM habit",
        )?;
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
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(habits)
    }

    /// Upsert today's completion state for a habit.
    /// Calling this multiple times in a day just overwrites the same row.
    pub fn set_completion(&self, habit_id: i32, completed: bool) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO habit_log (habit_id, date, completed)
             VALUES (?1, date('now'), ?2)
             ON CONFLICT(habit_id, date) DO UPDATE SET completed = excluded.completed",
            params![habit_id, completed as i32],
        )?;
        Ok(())
    }

    /// Whether the habit is marked complete today.
    pub fn is_completed_today(&self, habit_id: i32) -> Result<bool, Error> {
        let result = self.conn.query_row(
            "SELECT completed FROM habit_log WHERE habit_id = ?1 AND date = date('now')",
            params![habit_id],
            |row| row.get::<_, i32>(0),
        );
        match result {
            Ok(v) => Ok(v != 0),
            Err(Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Count consecutive completed days ending today (the streak).
    pub fn get_streak(&self, habit_id: i32) -> Result<i32, Error> {
        let mut stmt = self.conn.prepare(
            "SELECT date FROM habit_log
             WHERE habit_id = ?1 AND completed = 1
             ORDER BY date DESC",
        )?;
        let dates: Vec<String> = stmt
            .query_map(params![habit_id], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

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

    fn create_habit_table(&self) -> Result<(), Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS habit (
                id           INTEGER PRIMARY KEY,
                habit_name   TEXT    NOT NULL,
                streaks      INTEGER NOT NULL DEFAULT 0,
                daily_goal   INTEGER NOT NULL DEFAULT 0,
                weekly_goal  INTEGER NOT NULL DEFAULT 0,
                monthly_goal INTEGER NOT NULL DEFAULT 0,
                yearly_goal  INTEGER NOT NULL DEFAULT 0,
                created_at   TEXT    NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;
        Ok(())
    }

    fn create_habit_log_table(&self) -> Result<(), Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS habit_log (
                id        INTEGER PRIMARY KEY,
                habit_id  INTEGER NOT NULL REFERENCES habit(id) ON DELETE CASCADE,
                date      TEXT    NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                UNIQUE(habit_id, date)
            )",
            [],
        )?;
        Ok(())
    }
}
