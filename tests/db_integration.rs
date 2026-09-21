use chrono::Datelike;
use sprout_tui::model::habit::{HabitDb, HabitUpdate, NewHabit};
use sprout_tui::model::settings::SettingsDb;
use tempfile::NamedTempFile;

fn temp_db() -> (NamedTempFile, HabitDb) {
    let file = NamedTempFile::new().unwrap();
    let db = HabitDb::new(file.path()).unwrap();
    (file, db)
}

fn new_habit(name: &str, daily: i32, weekly: i32, monthly: i32, yearly: i32) -> NewHabit {
    NewHabit {
        name: name.to_string(),
        daily_goal: daily,
        weekly_goal: weekly,
        monthly_goal: monthly,
        yearly_goal: yearly,
    }
}

// ── Habit CRUD ────────────────────────────────────────────────────────────────

#[test]
fn create_habit_returns_correct_fields() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    assert_eq!(habit.name, "Reading");
    assert_eq!(habit.daily_goal, 30);
    assert_eq!(habit.weekly_goal, 0);
    assert_eq!(habit.monthly_goal, 0);
    assert_eq!(habit.yearly_goal, 0);
    assert!(habit.id > 0);
}

#[test]
fn create_binary_habit_all_goals_zero() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    assert_eq!(habit.daily_goal, 0);
    assert_eq!(habit.weekly_goal, 0);
    assert_eq!(habit.monthly_goal, 0);
    assert_eq!(habit.yearly_goal, 0);
}

#[test]
fn create_habit_with_all_goals() {
    let (_f, mut db) = temp_db();
    let habit = db
        .create_habit(&new_habit("Running", 5, 30, 100, 1000))
        .unwrap();
    assert_eq!(habit.daily_goal, 5);
    assert_eq!(habit.weekly_goal, 30);
    assert_eq!(habit.monthly_goal, 100);
    assert_eq!(habit.yearly_goal, 1000);
}

#[test]
fn get_all_habits_empty_initially() {
    let (_f, db) = temp_db();
    let habits = db.get_all_habits().unwrap();
    assert!(habits.is_empty());
}

#[test]
fn get_all_habits_returns_all_created() {
    let (_f, mut db) = temp_db();
    db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    db.create_habit(&new_habit("Meditation", 15, 0, 0, 0))
        .unwrap();
    let habits = db.get_all_habits().unwrap();
    assert_eq!(habits.len(), 3);
}

#[test]
fn delete_habit_removes_it_from_list() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.delete_habit(habit.id).unwrap();
    let habits = db.get_all_habits().unwrap();
    assert!(habits.is_empty());
}

#[test]
fn delete_habit_cascades_to_logs() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    db.delete_habit(habit.id).unwrap();
    let progress = db.get_daily_progress().unwrap();
    assert!(!progress.contains_key(&habit.id));
}

#[test]
fn update_habit_name() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reeding", 30, 0, 0, 0)).unwrap();
    let created_at = habit.created_at.clone();
    db.update_habit(
        &HabitUpdate {
            id: habit.id,
            name: "Reading".to_string(),
            daily_goal: 30,
            weekly_goal: 0,
            monthly_goal: 0,
            yearly_goal: 0,
        },
        created_at,
    )
    .unwrap();
    let habits = db.get_all_habits().unwrap();
    assert_eq!(habits[0].name, "Reading");
}

#[test]
fn update_habit_goals() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    let created_at = habit.created_at.clone();
    db.update_habit(
        &HabitUpdate {
            id: habit.id,
            name: "Reading".to_string(),
            daily_goal: 50,
            weekly_goal: 0,
            monthly_goal: 0,
            yearly_goal: 0,
        },
        created_at,
    )
    .unwrap();
    let habits = db.get_all_habits().unwrap();
    assert_eq!(habits[0].daily_goal, 50);
}

// ── Logging ───────────────────────────────────────────────────────────────────

#[test]
fn log_habit_progress_stores_correctly() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    let log = db.log_habit_progress(habit.id, 1, 30).unwrap();
    assert_eq!(log.habit_id, habit.id);
    assert_eq!(log.progress, 30);
    assert!(log.completed);
}

#[test]
fn log_binary_habit_progress_is_zero() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    let log = db.log_habit_progress(habit.id, 1, 0).unwrap();
    assert_eq!(log.progress, 0);
    assert!(log.completed);
}

#[test]
fn log_habit_twice_upserts() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 20).unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let progress = db.get_daily_progress().unwrap();
    assert_eq!(progress.get(&habit.id).copied().unwrap_or(0), 30);
}

#[test]
fn log_zero_marks_not_completed() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    let log = db.log_habit_progress(habit.id, 0, 0).unwrap();
    assert!(!log.completed);
}

// ── Completion ────────────────────────────────────────────────────────────────

#[test]
fn get_completed_habit_ids_today_empty_initially() {
    let (_f, db) = temp_db();
    let ids = db.get_completed_habit_ids_today().unwrap();
    assert!(ids.is_empty());
}

#[test]
fn get_completed_habit_ids_today_after_logging() {
    let (_f, mut db) = temp_db();
    let h1 = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    let h2 = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(h1.id, 1, 0).unwrap();
    db.log_habit_progress(h2.id, 0, 10).unwrap();
    let ids = db.get_completed_habit_ids_today().unwrap();
    assert!(ids.contains(&h1.id));
    assert!(!ids.contains(&h2.id));
}

// ── Progress ──────────────────────────────────────────────────────────────────

#[test]
fn daily_progress_empty_when_nothing_logged() {
    let (_f, db) = temp_db();
    let progress = db.get_daily_progress().unwrap();
    assert!(progress.is_empty());
}

#[test]
fn daily_progress_reflects_logged_value() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 25).unwrap();
    let progress = db.get_daily_progress().unwrap();
    assert_eq!(progress.get(&habit.id).copied().unwrap_or(0), 25);
}

#[test]
fn weekly_progress_includes_todays_log() {
    let (_f, mut db) = temp_db();
    let habit = db
        .create_habit(&new_habit("Reading", 30, 200, 0, 0))
        .unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let progress = db.get_weekly_progress().unwrap();
    assert_eq!(progress.get(&habit.id).copied().unwrap_or(0), 30);
}

#[test]
fn monthly_progress_includes_todays_log() {
    let (_f, mut db) = temp_db();
    let habit = db
        .create_habit(&new_habit("Reading", 30, 0, 600, 0))
        .unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let progress = db.get_monthly_progress().unwrap();
    assert_eq!(progress.get(&habit.id).copied().unwrap_or(0), 30);
}

#[test]
fn yearly_progress_includes_todays_log() {
    let (_f, mut db) = temp_db();
    let habit = db
        .create_habit(&new_habit("Reading", 30, 0, 0, 5000))
        .unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let progress = db.get_yearly_progress().unwrap();
    assert_eq!(progress.get(&habit.id).copied().unwrap_or(0), 30);
}

#[test]
fn multiple_habits_progress_tracked_independently() {
    let (_f, mut db) = temp_db();
    let h1 = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    let h2 = db
        .create_habit(&new_habit("Meditation", 15, 0, 0, 0))
        .unwrap();
    db.log_habit_progress(h1.id, 1, 30).unwrap();
    db.log_habit_progress(h2.id, 1, 10).unwrap();
    let progress = db.get_daily_progress().unwrap();
    assert_eq!(progress.get(&h1.id).copied().unwrap_or(0), 30);
    assert_eq!(progress.get(&h2.id).copied().unwrap_or(0), 10);
}

// ── Streaks ───────────────────────────────────────────────────────────────────

#[test]
fn streak_is_zero_when_not_logged() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    let streak = db.get_streak(habit.id).unwrap();
    assert_eq!(streak, 0);
}

#[test]
fn streak_is_one_after_logging_today() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 0).unwrap();
    let streak = db.get_streak(habit.id).unwrap();
    assert_eq!(streak, 1);
}

#[test]
fn best_streaks_empty_when_no_logs() {
    let (_f, db) = temp_db();
    let streaks = db.get_best_streaks().unwrap();
    assert!(streaks.is_empty());
}

#[test]
fn best_streaks_includes_habit_after_logging() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 0).unwrap();
    let streaks = db.get_best_streaks().unwrap();
    assert!(!streaks.is_empty());
    assert_eq!(streaks[0].habit_id, habit.id);
    assert_eq!(streaks[0].count, 1);
}

// ── Heatmap ───────────────────────────────────────────────────────────────────

#[test]
fn heatmap_data_empty_when_no_logs() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    let year = chrono::Local::now().year();
    let data = db.get_heatmap_data(habit.id, year).unwrap();
    assert!(data.is_empty());
}

#[test]
fn heatmap_data_contains_entry_after_logging() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let year = chrono::Local::now().year();
    let data = db.get_heatmap_data(habit.id, year).unwrap();
    assert_eq!(data.len(), 1);
    let (_, completed, progress) = &data[0];
    assert!(*completed);
    assert_eq!(*progress, 30);
}

#[test]
fn heatmap_year_habits_empty_when_no_logs() {
    let (_f, db) = temp_db();
    let result = db.get_heatmap_year_habits().unwrap();
    assert!(result.is_empty());
}

#[test]
fn heatmap_year_habits_contains_habit_after_logging() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let result = db.get_heatmap_year_habits().unwrap();
    assert!(!result.is_empty());
    let (_, ids) = &result[0];
    assert!(ids.contains(&habit.id));
}

// ── Active days ───────────────────────────────────────────────────────────────

#[test]
fn active_days_zero_when_no_logs() {
    let (_f, db) = temp_db();
    let count = db.get_active_days_count().unwrap();
    assert_eq!(count, 0);
}

#[test]
fn active_days_one_after_logging_today() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 0).unwrap();
    let count = db.get_active_days_count().unwrap();
    assert_eq!(count, 1);
}

#[test]
fn active_days_counts_once_per_day_across_habits() {
    let (_f, mut db) = temp_db();
    let h1 = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    let h2 = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(h1.id, 1, 0).unwrap();
    db.log_habit_progress(h2.id, 1, 30).unwrap();
    let count = db.get_active_days_count().unwrap();
    assert_eq!(count, 1);
}

// ── Weekly completion by day ──────────────────────────────────────────────────

#[test]
fn weekly_completion_all_zero_when_no_logs() {
    let (_f, db) = temp_db();
    let days = db.get_weekly_completion_by_day().unwrap();
    assert!(days.iter().all(|&d| d == 0));
}

// ── Settings ──────────────────────────────────────────────────────────────────

#[test]
fn settings_load_empty_initially() {
    let file = NamedTempFile::new().unwrap();
    let db = SettingsDb::new(file.path()).unwrap();
    let settings = db.load_settings().unwrap();
    assert!(settings.is_empty());
}

#[test]
fn settings_save_and_load_roundtrip() {
    let file = NamedTempFile::new().unwrap();
    let db = SettingsDb::new(file.path()).unwrap();
    db.save_setting("theme", "1").unwrap();
    let settings = db.load_settings().unwrap();
    assert_eq!(settings.get("theme").map(String::as_str), Some("1"));
}

#[test]
fn settings_upsert_overwrites_existing_key() {
    let file = NamedTempFile::new().unwrap();
    let db = SettingsDb::new(file.path()).unwrap();
    db.save_setting("theme", "0").unwrap();
    db.save_setting("theme", "2").unwrap();
    let settings = db.load_settings().unwrap();
    assert_eq!(settings.get("theme").map(String::as_str), Some("2"));
}

#[test]
fn settings_multiple_keys() {
    let file = NamedTempFile::new().unwrap();
    let db = SettingsDb::new(file.path()).unwrap();
    db.save_setting("theme", "1").unwrap();
    db.save_setting("cursor_blink", "0").unwrap();
    db.save_setting("notification_level", "2").unwrap();
    let settings = db.load_settings().unwrap();
    assert_eq!(settings.len(), 3);
    assert_eq!(settings.get("cursor_blink").map(String::as_str), Some("0"));
    assert_eq!(
        settings.get("notification_level").map(String::as_str),
        Some("2")
    );
}

// ── CLI validation ────────────────────────────────────────────────────────────

#[test]
fn cmd_log_rejects_negative_progress_for_goal_habit() {
    let (f, mut db) = temp_db();
    db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    let result = sprout_tui::cli::cmd_log(f.path(), "Reading".to_string(), -5);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("negative"));
}

#[test]
fn cmd_log_accepts_zero_progress_for_goal_habit() {
    let (f, mut db) = temp_db();
    db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    let result = sprout_tui::cli::cmd_log(f.path(), "Reading".to_string(), 0);
    assert!(result.is_ok());
}

// ── End-to-end flows ──────────────────────────────────────────────────────────

#[test]
fn flow_create_log_verify_completed() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    assert!(
        !db.get_completed_habit_ids_today()
            .unwrap()
            .contains(&habit.id)
    );
    db.log_habit_progress(habit.id, 1, 0).unwrap();
    let ids = db.get_completed_habit_ids_today().unwrap();
    assert!(ids.contains(&habit.id));
}

#[test]
fn flow_create_log_check_progress_then_delete() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let progress = db.get_daily_progress().unwrap();
    assert_eq!(progress.get(&habit.id).copied().unwrap_or(0), 30);
    db.delete_habit(habit.id).unwrap();
    assert!(db.get_all_habits().unwrap().is_empty());
    assert!(db.get_daily_progress().unwrap().is_empty());
}

#[test]
fn flow_log_update_relog() {
    let (_f, mut db) = temp_db();
    let habit = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    db.log_habit_progress(habit.id, 0, 10).unwrap();
    let p1 = db.get_daily_progress().unwrap();
    assert_eq!(p1.get(&habit.id).copied().unwrap_or(0), 10);
    db.log_habit_progress(habit.id, 1, 30).unwrap();
    let p2 = db.get_daily_progress().unwrap();
    assert_eq!(p2.get(&habit.id).copied().unwrap_or(0), 30);
    assert!(
        db.get_completed_habit_ids_today()
            .unwrap()
            .contains(&habit.id)
    );
}

#[test]
fn flow_multiple_habits_only_some_completed() {
    let (_f, mut db) = temp_db();
    let h1 = db.create_habit(&new_habit("Exercise", 0, 0, 0, 0)).unwrap();
    let h2 = db.create_habit(&new_habit("Reading", 30, 0, 0, 0)).unwrap();
    let h3 = db
        .create_habit(&new_habit("Meditation", 15, 0, 0, 0))
        .unwrap();
    db.log_habit_progress(h1.id, 1, 0).unwrap();
    db.log_habit_progress(h2.id, 1, 30).unwrap();
    let completed = db.get_completed_habit_ids_today().unwrap();
    assert!(completed.contains(&h1.id));
    assert!(completed.contains(&h2.id));
    assert!(!completed.contains(&h3.id));
    assert_eq!(completed.len(), 2);
}
