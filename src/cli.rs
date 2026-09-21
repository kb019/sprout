use std::path::Path;
use std::{collections::HashSet, io::Write};

use anyhow::{Context, Result, bail};
use chrono::{Datelike, Duration, Local, NaiveDate};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Color, Stylize},
    terminal,
};

use crate::model::habit::{HabitDb, HabitUpdate, NewHabit};

pub fn cmd_add(
    db_path: &Path,
    name: String,
    daily_goal: i32,
    weekly_goal: i32,
    monthly_goal: i32,
    yearly_goal: i32,
) -> Result<()> {
    let mut db = HabitDb::new(db_path)?;
    let habit = db.create_habit(&NewHabit {
        name,
        daily_goal,
        weekly_goal,
        monthly_goal,
        yearly_goal,
    })?;
    println!("  Created  {}", habit.name.as_str().bold());
    if habit.daily_goal > 0 {
        println!(
            "  Daily goal:    {}",
            habit.daily_goal.to_string().as_str().green()
        );
    }
    if habit.weekly_goal > 0 {
        println!(
            "  Weekly goal:   {}",
            habit.weekly_goal.to_string().as_str().green()
        );
    }
    if habit.monthly_goal > 0 {
        println!(
            "  Monthly goal:  {}",
            habit.monthly_goal.to_string().as_str().green()
        );
    }
    if habit.yearly_goal > 0 {
        println!(
            "  Yearly goal:   {}",
            habit.yearly_goal.to_string().as_str().green()
        );
    }
    Ok(())
}

pub fn cmd_list(db_path: &Path) -> Result<()> {
    let db = HabitDb::new(db_path)?;
    let habits = db.get_all_habits()?;

    if habits.is_empty() {
        println!("  No habits yet. Run `sprout add` to create one.");
        return Ok(());
    }

    let daily_progress = db.get_daily_progress()?;
    let weekly_progress = db.get_weekly_progress()?;
    let monthly_progress = db.get_monthly_progress()?;
    let yearly_progress = db.get_yearly_progress()?;
    let completed_today: HashSet<i32> = db.get_completed_habit_ids_today()?.into_iter().collect();

    println!();
    println!(
        "  {}  {}  {}  {}  {}  {}",
        format!("{:<4}", "ID").dim(),
        format!("{:<20}", "Name").dim(),
        format!("{:>9}", "Today").dim(),
        format!("{:>9}", "Weekly").dim(),
        format!("{:>10}", "Monthly").dim(),
        format!("{:>10}", "Yearly").dim(),
    );
    println!("  ────  ────────────────────  ─────────  ─────────  ──────────  ──────────");
    for h in &habits {
        let today = daily_progress.get(&h.id).copied().unwrap_or(0);
        let is_done = completed_today.contains(&h.id);
        let today_str = if h.daily_goal > 0 {
            format!("{}/{}", today, h.daily_goal)
        } else {
            today.to_string()
        };
        let today_col = if is_done {
            format!("{:>9}", today_str).green().to_string()
        } else {
            format!("{:>9}", today_str).dim().to_string()
        };

        let weekly = weekly_progress.get(&h.id).copied().unwrap_or(0);
        let weekly_str = if h.weekly_goal > 0 {
            format!("{}/{}", weekly, h.weekly_goal)
        } else {
            "─".to_string()
        };

        let monthly = monthly_progress.get(&h.id).copied().unwrap_or(0);
        let monthly_str = if h.monthly_goal > 0 {
            format!("{}/{}", monthly, h.monthly_goal)
        } else {
            "─".to_string()
        };

        let yearly = yearly_progress.get(&h.id).copied().unwrap_or(0);
        let yearly_str = if h.yearly_goal > 0 {
            format!("{}/{}", yearly, h.yearly_goal)
        } else {
            "─".to_string()
        };

        println!(
            "  {}  {}  {}  {}  {}  {}",
            format!("{:<4}", h.id).dim(),
            format!("{:<20}", h.name).bold(),
            today_col,
            format!("{:>9}", weekly_str).dim(),
            format!("{:>10}", monthly_str).dim(),
            format!("{:>10}", yearly_str).dim(),
        );
    }
    println!();
    Ok(())
}

pub fn cmd_status(db_path: &Path) -> Result<()> {
    let db = HabitDb::new(db_path)?;
    let habits = db.get_all_habits()?;

    if habits.is_empty() {
        println!("  No habits yet. Run `sprout add` to create one.");
        return Ok(());
    }

    let today = Local::now().format("%A, %Y-%m-%d").to_string();
    let daily_progress = db.get_daily_progress()?;
    let completed_today: HashSet<i32> = db.get_completed_habit_ids_today()?.into_iter().collect();

    let done_count = habits
        .iter()
        .filter(|h| completed_today.contains(&h.id))
        .count();

    println!();
    println!(
        "  {}  {}  {}",
        "Today".dim(),
        today.bold(),
        format!("({}/{})", done_count, habits.len()).dim(),
    );
    println!("  {}", "─".repeat(40).dim());
    for h in &habits {
        let progress = daily_progress.get(&h.id).copied().unwrap_or(0);
        let is_done = completed_today.contains(&h.id);
        let mark = if is_done {
            "✓".to_string().green().bold().to_string()
        } else {
            "✗".to_string().red().to_string()
        };
        let goal_str = if h.daily_goal > 0 {
            format!("{} / {}", progress, h.daily_goal)
        } else if progress > 0 {
            progress.to_string()
        } else if is_done {
            "logged".to_string()
        } else {
            "not logged".to_string()
        };

        let styled_goal = if is_done {
            goal_str.green().to_string()
        } else {
            goal_str.dim().to_string()
        };
        println!(
            "  {}  {}  {}",
            mark,
            format!("{:<22}", h.name).bold(),
            styled_goal,
        );
    }
    println!();
    Ok(())
}

pub fn cmd_log(db_path: &Path, habit_name: String, value: i32) -> Result<()> {
    let mut db = HabitDb::new(db_path)?;
    let (habit_id, display_name, daily_goal, is_binary) = {
        let habits = db.get_all_habits()?;
        let h = habits
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(&habit_name))
            .with_context(|| {
                format!(
                    "Habit '{}' not found. Run `sprout list` to see your habits.",
                    habit_name
                )
            })?;
        let binary =
            h.daily_goal == 0 && h.weekly_goal == 0 && h.monthly_goal == 0 && h.yearly_goal == 0;
        (h.id, h.name.clone(), h.daily_goal, binary)
    };

    if is_binary && value != 0 && value != 1 {
        bail!(
            "'{}' is a binary habit. Only 0 (not done) or 1 (done) can be logged.",
            display_name
        );
    }

    if !is_binary && value < 0 {
        bail!("Progress cannot be negative.");
    }

    let completed: i32 = if daily_goal > 0 && value >= daily_goal || daily_goal == 0 && value > 0 {
        1
    } else {
        0
    };

    let progress = if is_binary { 0 } else { value };
    db.log_habit_progress(habit_id, completed, progress)?;

    if daily_goal > 0 {
        let remaining = daily_goal - value;
        let status = if completed == 1 {
            "  ✓".to_string().green().bold().to_string()
        } else if remaining > 0 {
            format!("  ({} to go)", remaining).dim().to_string()
        } else {
            String::new()
        };
        println!(
            "  Logged {} for {}  →  {}/{}{}",
            value.to_string().as_str().green().bold(),
            display_name.as_str().bold(),
            value,
            daily_goal,
            status,
        );
    } else {
        println!(
            "  Logged {} for {}",
            value.to_string().as_str().green().bold(),
            display_name.as_str().bold(),
        );
    }
    Ok(())
}

pub fn cmd_delete(db_path: &Path, name: String, yes: bool) -> Result<()> {
    let mut db = HabitDb::new(db_path)?;
    let (habit_id, habit_name) = {
        let habits = db.get_all_habits()?;
        let h = habits
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(&name))
            .with_context(|| {
                format!(
                    "Habit '{}' not found. Run `sprout list` to see your habits.",
                    name
                )
            })?;
        (h.id, h.name.clone())
    };

    if !yes {
        print!(
            "  Delete {} and all its history? [y/N] ",
            habit_name.as_str().bold()
        );
        std::io::stdout().flush()?;

        terminal::enable_raw_mode()?;
        // Drain any buffered events (e.g. the Enter that submitted the command).
        while event::poll(std::time::Duration::from_millis(50)).unwrap_or(false) {
            let _ = event::read();
        }
        let confirmed = loop {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => break true,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break false;
                    }
                    KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Enter => {
                        break false;
                    }
                    _ => {}
                }
            }
        };
        terminal::disable_raw_mode()?;
        println!();

        if !confirmed {
            println!("  Cancelled.");
            return Ok(());
        }
    }

    db.delete_habit(habit_id)?;
    println!("  Deleted  {}", habit_name.as_str().bold());
    Ok(())
}

pub fn cmd_edit(
    db_path: &Path,
    name: String,
    rename: Option<String>,
    daily_goal: Option<i32>,
    weekly_goal: Option<i32>,
    monthly_goal: Option<i32>,
    yearly_goal: Option<i32>,
) -> Result<()> {
    if rename.is_none()
        && daily_goal.is_none()
        && weekly_goal.is_none()
        && monthly_goal.is_none()
        && yearly_goal.is_none()
    {
        bail!(
            "Nothing to update. Use --rename, --daily-goal, --weekly-goal, --monthly-goal, or --yearly-goal."
        );
    }

    let mut db = HabitDb::new(db_path)?;
    let (habit_id, old_name, old_daily, old_weekly, old_monthly, old_yearly, created_at) = {
        let habits = db.get_all_habits()?;
        let h = habits
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(&name))
            .with_context(|| {
                format!(
                    "Habit '{}' not found. Run `sprout list` to see your habits.",
                    name
                )
            })?;
        (
            h.id,
            h.name.clone(),
            h.daily_goal,
            h.weekly_goal,
            h.monthly_goal,
            h.yearly_goal,
            h.created_at.clone(),
        )
    };

    // Binary habit: all goals are zero (pure done/not-done tracking).
    let is_binary = old_daily == 0 && old_weekly == 0 && old_monthly == 0 && old_yearly == 0;
    let any_goal: bool = daily_goal.is_some()
        || weekly_goal.is_some()
        || monthly_goal.is_some()
        || yearly_goal.is_some();
    if is_binary && any_goal {
        bail!(
            "'{}' is a binary habit (no goals). Goals cannot be added via edit.",
            old_name
        );
    }
    // Only goals that existed at creation can be changed.
    if daily_goal.is_some() && old_daily == 0 {
        bail!(
            "'{}' had no daily goal at creation. Only existing goals can be edited.",
            old_name
        );
    }
    if weekly_goal.is_some() && old_weekly == 0 {
        bail!(
            "'{}' had no weekly goal at creation. Only existing goals can be edited.",
            old_name
        );
    }
    if monthly_goal.is_some() && old_monthly == 0 {
        bail!(
            "'{}' had no monthly goal at creation. Only existing goals can be edited.",
            old_name
        );
    }
    if yearly_goal.is_some() && old_yearly == 0 {
        bail!(
            "'{}' had no yearly goal at creation. Only existing goals can be edited.",
            old_name
        );
    }

    let new_name = rename.as_deref().unwrap_or(&old_name).to_string();
    let new_daily = daily_goal.unwrap_or(old_daily);
    let new_weekly = weekly_goal.unwrap_or(old_weekly);
    let new_monthly = monthly_goal.unwrap_or(old_monthly);
    let new_yearly = yearly_goal.unwrap_or(old_yearly);

    db.update_habit(
        &HabitUpdate {
            id: habit_id,
            name: new_name.clone(),
            daily_goal: new_daily,
            weekly_goal: new_weekly,
            monthly_goal: new_monthly,
            yearly_goal: new_yearly,
        },
        created_at,
    )?;

    println!("  Updated  {}", new_name.as_str().bold());
    if rename.is_some() {
        println!(
            "  Name:          {}  →  {}",
            old_name.as_str().dim(),
            new_name.as_str().green()
        );
    }
    if daily_goal.is_some() {
        println!(
            "  Daily goal:    {}  →  {}",
            old_daily.to_string().as_str().dim(),
            new_daily.to_string().as_str().green()
        );
    }
    if weekly_goal.is_some() {
        println!(
            "  Weekly goal:   {}  →  {}",
            old_weekly.to_string().as_str().dim(),
            new_weekly.to_string().as_str().green()
        );
    }
    if monthly_goal.is_some() {
        println!(
            "  Monthly goal:  {}  →  {}",
            old_monthly.to_string().as_str().dim(),
            new_monthly.to_string().as_str().green()
        );
    }
    if yearly_goal.is_some() {
        println!(
            "  Yearly goal:   {}  →  {}",
            old_yearly.to_string().as_str().dim(),
            new_yearly.to_string().as_str().green()
        );
    }
    Ok(())
}

pub fn cmd_streak(db_path: &Path, name: Option<String>) -> Result<()> {
    let db = HabitDb::new(db_path)?;
    let all_habits = db.get_all_habits()?;

    if all_habits.is_empty() {
        println!("  No habits yet. Run `sprout add` to create one.");
        return Ok(());
    }

    let filter = name.as_deref().map(str::to_lowercase);
    let habits: Vec<_> = if let Some(ref n) = filter {
        all_habits
            .iter()
            .filter(|h| h.name.to_lowercase() == *n)
            .collect()
    } else {
        all_habits.iter().collect()
    };

    if habits.is_empty() {
        bail!(
            "Habit '{}' not found. Run `sprout list` to see your habits.",
            name.unwrap_or_default()
        );
    }

    let best_streaks = db.get_best_streaks()?;

    println!();
    println!(
        "  {}  {}  {}",
        format!("{:<22}", "Habit").dim(),
        format!("{:>9}", "Current").dim(),
        format!("{:>10}", "Best").dim(),
    );
    println!("  ──────────────────────  ─────────  ──────────");
    for h in &habits {
        let current = db.get_streak(h.id)?;
        let best_entry = best_streaks.iter().find(|b| b.habit_id == h.id);
        let best = best_entry.map(|b| b.count).unwrap_or(0);

        let current_str = if current == 1 {
            "1 day".to_string()
        } else {
            format!("{} days", current)
        };
        let best_str = if best == 1 {
            "1 day".to_string()
        } else if best > 0 {
            format!("{} days", best)
        } else {
            "─".to_string()
        };

        let current_col = if current > 0 {
            format!("{:>9}", current_str).green().to_string()
        } else {
            format!("{:>9}", current_str).dim().to_string()
        };
        let best_col = if best > 0 {
            format!("{:>10}", best_str).green().to_string()
        } else {
            format!("{:>10}", best_str).dim().to_string()
        };

        println!(
            "  {}  {}  {}",
            format!("{:<22}", h.name).bold(),
            current_col,
            best_col,
        );
        if let Some(entry) = best_entry {
            println!(
                "  {}  {}  {}",
                " ".repeat(22),
                " ".repeat(9),
                format!("{} → {}", entry.start, entry.end).dim(),
            );
        }
    }
    println!();
    Ok(())
}

pub fn cmd_heatmap(db_path: &Path, habit_name: String, year: Option<i32>) -> Result<()> {
    let db = HabitDb::new(db_path)?;
    let (habit_id, display_name, _daily_goal, is_binary) = {
        let habits = db.get_all_habits()?;
        let h = habits
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(&habit_name))
            .with_context(|| {
                format!(
                    "Habit '{}' not found. Run `sprout list` to see your habits.",
                    habit_name
                )
            })?;
        let binary =
            h.daily_goal == 0 && h.weekly_goal == 0 && h.monthly_goal == 0 && h.yearly_goal == 0;
        (h.id, h.name.clone(), h.daily_goal, binary)
    };

    let target_year = year.unwrap_or_else(|| Local::now().year());
    let data = db.get_heatmap_data(habit_id, target_year)?;

    // Same min-max normalization as the TUI (src/update/mod.rs).
    let max_p = data.iter().map(|(_, _, p)| *p).max().unwrap_or(0);
    let min_p = data.iter().map(|(_, _, p)| *p).min().unwrap_or(0);
    let mut intensity_map: std::collections::HashMap<NaiveDate, u8> =
        std::collections::HashMap::new();
    for (date_str, completed, progress) in &data {
        if let Ok(date) = date_str.parse::<NaiveDate>() {
            let intensity = if max_p == 0 || max_p == min_p {
                if *completed { 4 } else { 0 }
            } else {
                ((*progress - min_p) as f32 / (max_p - min_p) as f32 * 4.0).min(4.0) as u8
            };
            intensity_map.insert(date, intensity);
        }
    }

    let year_start = NaiveDate::from_ymd_opt(target_year, 1, 1).context("invalid year")?;
    let year_end = NaiveDate::from_ymd_opt(target_year, 12, 31).context("invalid year")?;
    let offset = year_start.weekday().num_days_from_monday() as i64;
    let grid_start = year_start - Duration::days(offset);
    let total_weeks = (((year_end - grid_start).num_days() + 1) as usize).div_ceil(7);

    const MONTH_ABBR: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut month_bytes = vec![b' '; total_weeks * 2];
    let mut prev_month = 0u32;
    for week in 0..total_weeks {
        let wd = grid_start + Duration::days(week as i64 * 7);
        let m = wd.month();
        if wd.year() == target_year && m != prev_month {
            let label = MONTH_ABBR[(m - 1) as usize].as_bytes();
            let pos = week * 2;
            for (i, &b) in label.iter().enumerate() {
                if pos + i < month_bytes.len() {
                    month_bytes[pos + i] = b;
                }
            }
            prev_month = m;
        }
    }

    let active_days = intensity_map.values().filter(|&&v| v > 0).count();

    const HEATMAP_RGB: [(u8, u8, u8); 5] = [
        (23, 35, 27),
        (14, 68, 41),
        (28, 107, 61),
        (47, 161, 85),
        (75, 227, 116),
    ];
    let colored_square = |level: usize| -> String {
        let (r, g, b) = HEATMAP_RGB[level];
        "■ ".with(Color::Rgb { r, g, b }).to_string()
    };

    // prefix "  Mon  " = 7 chars; each cell "■ " = 2 display cols.
    let term_width = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80);
    let max_weeks = (term_width.saturating_sub(7)) / 2;

    // Precompute the week-index span for each month (0-indexed, 0=Jan).
    let month_week_ranges: Vec<(usize, usize)> = (1u32..=12)
        .map(|m| {
            let ms = NaiveDate::from_ymd_opt(target_year, m, 1).unwrap();
            let me = if m == 12 {
                year_end
            } else {
                NaiveDate::from_ymd_opt(target_year, m + 1, 1).unwrap() - Duration::days(1)
            };
            let ws = ((ms - grid_start).num_days().max(0) as usize) / 7;
            let we = ((me - grid_start).num_days().max(0) as usize) / 7;
            (ws, we)
        })
        .collect();

    // Select which months to show: fill left from current month first, then right —
    // same priority logic as the TUI heatmap widget.
    let (show_start_week, show_end_week) = if max_weeks >= total_weeks {
        (0, total_weeks)
    } else {
        let cur_idx = (Local::now().month() as usize - 1).min(11);
        let mut left = cur_idx;
        let mut right = cur_idx;

        while left > 0
            && month_week_ranges[right].1 + 1 - month_week_ranges[left - 1].0 <= max_weeks
        {
            left -= 1;
        }
        while right < 11
            && month_week_ranges[right + 1].1 + 1 - month_week_ranges[left].0 <= max_weeks
        {
            right += 1;
        }

        (month_week_ranges[left].0, month_week_ranges[right].1 + 1)
    };

    println!();
    println!(
        "  {}  -  {}",
        display_name.as_str().bold(),
        target_year.to_string().as_str().dim()
    );
    println!();

    let month_label: String = month_bytes[show_start_week * 2..show_end_week * 2]
        .iter()
        .map(|&b| b as char)
        .collect();
    println!("       {}", month_label.as_str().dim());

    const DAY_LABELS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for (dow, label) in DAY_LABELS.iter().enumerate() {
        let mut row = format!("  {}  ", label.dim());
        for week in show_start_week..show_end_week {
            let date = grid_start + Duration::days((week * 7 + dow) as i64);
            if date < year_start || date > year_end {
                row.push_str("  ");
            } else {
                let intensity = intensity_map.get(&date).copied().unwrap_or(0);
                row.push_str(&colored_square(intensity as usize));
            }
        }
        println!("{}", row);
    }

    println!();
    print!("  ");
    if is_binary {
        print!("{}not done  {}done  ", colored_square(0), colored_square(4));
    } else {
        for (i, label) in ["none", "low", "mid", "high", "done"].iter().enumerate() {
            print!("{}{}  ", colored_square(i), label);
        }
    }
    println!();
    println!(
        "  {} active days in {}",
        active_days.to_string().as_str().green(),
        target_year
    );
    println!();
    Ok(())
}
