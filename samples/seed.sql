-- Sample seed data — all dates are relative to today via SQLite date functions.
-- Run via: sprout sample
--
-- Habit roster (5 total, phased in by year):
--   Year 1 (1095–731 days ago): Reading, Exercise, Meditation          (3 habits)
--   Year 2  (730–366 days ago): + Coding                               (4 habits)
--   Year 3   (365–8 days ago) : + Journaling                           (5 habits)
--   Last 7 days (7–1 days ago): explicit entries with real progress
--
-- Reading is the increasing habit: 50 % → 67 % → 80 % completion across years.
-- Exercise / Journaling are done/not-done (progress = 0, completed flag only).
-- Meditation / Coding carry minute-based progress values.

CREATE TABLE IF NOT EXISTS habit (
    id           INTEGER PRIMARY KEY,
    habit_name   TEXT    NOT NULL,
    streaks      INTEGER NOT NULL DEFAULT 0,
    daily_goal   INTEGER NOT NULL DEFAULT 0,
    weekly_goal  INTEGER NOT NULL DEFAULT 0,
    monthly_goal INTEGER NOT NULL DEFAULT 0,
    yearly_goal  INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE TABLE IF NOT EXISTS habit_log (
    id        INTEGER PRIMARY KEY,
    habit_id  INTEGER NOT NULL REFERENCES habit(id) ON DELETE CASCADE,
    date      TEXT    NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    progress  INTEGER NOT NULL DEFAULT 0,
    UNIQUE(habit_id, date)
);

INSERT INTO habit (id, habit_name, streaks, daily_goal, weekly_goal, monthly_goal, yearly_goal, created_at)
VALUES
  (1, 'Reading',    0, 30,  0, 0, 0, datetime('now', 'localtime', '-1100 days')),
  (2, 'Exercise',   0,  0,  0, 0, 0, datetime('now', 'localtime', '-1100 days')),
  (3, 'Meditation', 0, 15,  0, 0, 0, datetime('now', 'localtime', '-1100 days')),
  (4, 'Coding',     0, 60,  0, 0, 0, datetime('now', 'localtime',  '-735 days')),
  (5, 'Journaling', 0,  0,  0, 0, 0, datetime('now', 'localtime',  '-370 days'));

-- ============================================================
-- YEAR 1  (1095 – 731 days ago)
-- Reading 50% | Exercise 40% | Meditation 40%
-- ============================================================

-- Reading: ~50 % done, 30–39 pages
WITH RECURSIVE cnt(n) AS (SELECT 731 UNION ALL SELECT n+1 FROM cnt WHERE n < 1095)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 1,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 2 = 0 THEN 0 ELSE 1 END,
       CASE WHEN n % 2 = 0 THEN 0 ELSE 30 + (n % 10) END
FROM cnt;

-- Exercise: ~40 % done/not-done
WITH RECURSIVE cnt(n) AS (SELECT 731 UNION ALL SELECT n+1 FROM cnt WHERE n < 1095)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 2,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 5 IN (0, 1, 2) THEN 0 ELSE 1 END,
       0
FROM cnt;

-- Meditation: ~40 %, 15–24 min
WITH RECURSIVE cnt(n) AS (SELECT 731 UNION ALL SELECT n+1 FROM cnt WHERE n < 1095)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 3,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 5 IN (0, 1, 2) THEN 0 ELSE 1 END,
       CASE WHEN n % 5 IN (0, 1, 2) THEN 0 ELSE 15 + (n % 10) END
FROM cnt;

-- ============================================================
-- YEAR 2  (730 – 366 days ago)
-- Reading 67% ↑ | Exercise 55% ↑ | Meditation 60% ↑ | Coding 50% (new)
-- ============================================================

-- Reading: ~67 % (INCREASING), 32–43 pages
WITH RECURSIVE cnt(n) AS (SELECT 366 UNION ALL SELECT n+1 FROM cnt WHERE n < 730)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 1,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 3 = 0 THEN 0 ELSE 1 END,
       CASE WHEN n % 3 = 0 THEN 0 ELSE 32 + (n % 12) END
FROM cnt;

-- Exercise: ~55 %
WITH RECURSIVE cnt(n) AS (SELECT 366 UNION ALL SELECT n+1 FROM cnt WHERE n < 730)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 2,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 9 IN (0, 1, 2, 3) THEN 0 ELSE 1 END,
       0
FROM cnt;

-- Meditation: ~60 %, 15–24 min
WITH RECURSIVE cnt(n) AS (SELECT 366 UNION ALL SELECT n+1 FROM cnt WHERE n < 730)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 3,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 5 IN (0, 1) THEN 0 ELSE 1 END,
       CASE WHEN n % 5 IN (0, 1) THEN 0 ELSE 15 + (n % 10) END
FROM cnt;

-- Coding (new): ~50 %, 60–89 min
WITH RECURSIVE cnt(n) AS (SELECT 366 UNION ALL SELECT n+1 FROM cnt WHERE n < 730)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 4,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 2 = 0 THEN 0 ELSE 1 END,
       CASE WHEN n % 2 = 0 THEN 0 ELSE 60 + (n % 30) END
FROM cnt;

-- ============================================================
-- YEAR 3  (365 – 8 days ago)
-- Reading 80% ↑ | Exercise 67% ↑ | Meditation 75% ↑ | Coding 67% ↑ | Journaling 60% (new)
-- ============================================================

-- Reading: ~80 % (INCREASING), 30–44 pages
WITH RECURSIVE cnt(n) AS (SELECT 8 UNION ALL SELECT n+1 FROM cnt WHERE n < 365)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 1,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 5 = 0 THEN 0 ELSE 1 END,
       CASE WHEN n % 5 = 0 THEN 0 ELSE 30 + (n % 15) END
FROM cnt;

-- Exercise: ~67 %
WITH RECURSIVE cnt(n) AS (SELECT 8 UNION ALL SELECT n+1 FROM cnt WHERE n < 365)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 2,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 3 = 0 THEN 0 ELSE 1 END,
       0
FROM cnt;

-- Meditation: ~75 %, 15–24 min
WITH RECURSIVE cnt(n) AS (SELECT 8 UNION ALL SELECT n+1 FROM cnt WHERE n < 365)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 3,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 4 = 0 THEN 0 ELSE 1 END,
       CASE WHEN n % 4 = 0 THEN 0 ELSE 15 + (n % 10) END
FROM cnt;

-- Coding: ~67 %, 60–89 min
WITH RECURSIVE cnt(n) AS (SELECT 8 UNION ALL SELECT n+1 FROM cnt WHERE n < 365)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 4,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 3 = 1 THEN 0 ELSE 1 END,
       CASE WHEN n % 3 = 1 THEN 0 ELSE 60 + (n % 30) END
FROM cnt;

-- Journaling (new): ~60 % done/not-done
WITH RECURSIVE cnt(n) AS (SELECT 8 UNION ALL SELECT n+1 FROM cnt WHERE n < 365)
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress)
SELECT 5,
       date('now', 'localtime', '-' || n || ' days'),
       CASE WHEN n % 5 IN (0, 1) THEN 0 ELSE 1 END,
       0
FROM cnt;

-- ============================================================
-- LAST 7 DAYS (days 7 – 1 ago): explicit entries with realistic progress
-- ============================================================
INSERT OR IGNORE INTO habit_log (habit_id, date, completed, progress) VALUES
  -- 7 days ago
  (1, date('now', 'localtime', '-7 days'), 1, 30),
  (2, date('now', 'localtime', '-7 days'), 1,  0),
  (3, date('now', 'localtime', '-7 days'), 1, 17),
  (4, date('now', 'localtime', '-7 days'), 1, 63),
  (5, date('now', 'localtime', '-7 days'), 1,  0),
  -- 6 days ago
  (1, date('now', 'localtime', '-6 days'), 1, 35),
  (2, date('now', 'localtime', '-6 days'), 0,  0),
  (3, date('now', 'localtime', '-6 days'), 1, 22),
  (4, date('now', 'localtime', '-6 days'), 1, 75),
  (5, date('now', 'localtime', '-6 days'), 0,  0),
  -- 5 days ago
  (1, date('now', 'localtime', '-5 days'), 1, 32),
  (2, date('now', 'localtime', '-5 days'), 1,  0),
  (3, date('now', 'localtime', '-5 days'), 1, 16),
  (4, date('now', 'localtime', '-5 days'), 1, 67),
  (5, date('now', 'localtime', '-5 days'), 1,  0),
  -- 4 days ago
  (1, date('now', 'localtime', '-4 days'), 0,  0),
  (2, date('now', 'localtime', '-4 days'), 1,  0),
  (3, date('now', 'localtime', '-4 days'), 1, 20),
  (4, date('now', 'localtime', '-4 days'), 0,  0),
  (5, date('now', 'localtime', '-4 days'), 1,  0),
  -- 3 days ago
  (1, date('now', 'localtime', '-3 days'), 1, 38),
  (2, date('now', 'localtime', '-3 days'), 1,  0),
  (3, date('now', 'localtime', '-3 days'), 0,  0),
  (4, date('now', 'localtime', '-3 days'), 1, 80),
  (5, date('now', 'localtime', '-3 days'), 0,  0),
  -- 2 days ago
  (1, date('now', 'localtime', '-2 days'), 1, 31),
  (2, date('now', 'localtime', '-2 days'), 0,  0),
  (3, date('now', 'localtime', '-2 days'), 1, 15),
  (4, date('now', 'localtime', '-2 days'), 1, 65),
  (5, date('now', 'localtime', '-2 days'), 1,  0),
  -- yesterday
  (1, date('now', 'localtime', '-1 days'), 1, 34),
  (2, date('now', 'localtime', '-1 days'), 1,  0),
  (3, date('now', 'localtime', '-1 days'), 1, 18),
  (4, date('now', 'localtime', '-1 days'), 1, 72),
  (5, date('now', 'localtime', '-1 days'), 1,  0);
