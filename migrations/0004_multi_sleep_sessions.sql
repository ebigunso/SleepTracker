-- Multi-session support (wake-date based)
-- Safety checklist (recommended before applying):
-- 1) Backup DB: `.backup sleeptracker_pre_multi_sessions.sqlite`
-- 2) Record baseline totals:
--    - SELECT COUNT(*) AS session_count FROM sleep_sessions;
--    - SELECT SUM(duration_min) AS total_duration FROM sleep_metrics;
--    - SELECT date AS wake_date, SUM(duration_min) FROM sleep_sessions s
--      JOIN sleep_metrics m ON m.session_id = s.id GROUP BY date;
-- 3) After migration, re-run the same queries using session_date and compare.

-- Store single-user timezone setting (default: Asia/Tokyo)
CREATE TABLE IF NOT EXISTS app_settings (
    key     TEXT PRIMARY KEY,
    value   TEXT NOT NULL
);

INSERT OR IGNORE INTO app_settings(key, value)
VALUES ('user_timezone', 'Asia/Tokyo');

-- Add wake-derived session date (nullable during migration)
ALTER TABLE sleep_sessions ADD COLUMN session_date DATE;
UPDATE sleep_sessions
   SET session_date = date
 WHERE session_date IS NULL;

CREATE INDEX IF NOT EXISTS idx_sleep_sessions_session_date
    ON sleep_sessions(session_date);

-- Prevent overlapping sessions (end == start is also forbidden)
DROP TRIGGER IF EXISTS sleep_sessions_no_overlap_insert;
DROP TRIGGER IF EXISTS sleep_sessions_no_overlap_update;

CREATE TRIGGER sleep_sessions_no_overlap_insert
BEFORE INSERT ON sleep_sessions
FOR EACH ROW
BEGIN
    SELECT
        CASE
            WHEN EXISTS (
                SELECT 1
                FROM sleep_sessions s
                WHERE
                    (
                        datetime(COALESCE(NEW.session_date, NEW.date) || ' ' || NEW.wake_time) >=
                        CASE
                            WHEN s.bed_time > s.wake_time
                                THEN datetime(COALESCE(s.session_date, s.date) || ' ' || s.bed_time, '-1 day')
                            ELSE datetime(COALESCE(s.session_date, s.date) || ' ' || s.bed_time)
                        END
                        AND
                        CASE
                            WHEN NEW.bed_time > NEW.wake_time
                                THEN datetime(COALESCE(NEW.session_date, NEW.date) || ' ' || NEW.bed_time, '-1 day')
                            ELSE datetime(COALESCE(NEW.session_date, NEW.date) || ' ' || NEW.bed_time)
                        END <= datetime(COALESCE(s.session_date, s.date) || ' ' || s.wake_time)
                    )
            )
            THEN RAISE(ABORT, 'sleep session overlaps existing session')
        END;
END;

CREATE TRIGGER sleep_sessions_no_overlap_update
BEFORE UPDATE ON sleep_sessions
FOR EACH ROW
BEGIN
    SELECT
        CASE
            WHEN EXISTS (
                SELECT 1
                FROM sleep_sessions s
                WHERE s.id != NEW.id
                  AND (
                        datetime(COALESCE(NEW.session_date, NEW.date) || ' ' || NEW.wake_time) >=
                        CASE
                            WHEN s.bed_time > s.wake_time
                                THEN datetime(COALESCE(s.session_date, s.date) || ' ' || s.bed_time, '-1 day')
                            ELSE datetime(COALESCE(s.session_date, s.date) || ' ' || s.bed_time)
                        END
                        AND
                        CASE
                            WHEN NEW.bed_time > NEW.wake_time
                                THEN datetime(COALESCE(NEW.session_date, NEW.date) || ' ' || NEW.bed_time, '-1 day')
                            ELSE datetime(COALESCE(NEW.session_date, NEW.date) || ' ' || NEW.bed_time)
                        END <= datetime(COALESCE(s.session_date, s.date) || ' ' || s.wake_time)
                    )
            )
            THEN RAISE(ABORT, 'sleep session overlaps existing session')
        END;
END;

-- Update daily view to aggregate multiple sessions per wake date
DROP VIEW IF EXISTS v_daily_sleep;
CREATE VIEW v_daily_sleep AS
SELECT
    MIN(base.id) AS id,
    base.wake_date,
    time(MIN(base.bed_dt)) AS bed_time,
    time(MAX(base.wake_dt)) AS wake_time,
    CAST(AVG(base.latency_min) AS INTEGER) AS latency_min,
    SUM(base.awakenings) AS awakenings,
    CAST(AVG(base.quality) AS INTEGER) AS quality,
    SUM(base.duration_min) AS duration_min,
    COUNT(*) AS session_count
FROM (
    SELECT
        s.id,
        COALESCE(s.session_date, s.date) AS wake_date,
        CASE
            WHEN s.bed_time > s.wake_time
                THEN datetime(COALESCE(s.session_date, s.date) || ' ' || s.bed_time, '-1 day')
            ELSE datetime(COALESCE(s.session_date, s.date) || ' ' || s.bed_time)
        END AS bed_dt,
        datetime(COALESCE(s.session_date, s.date) || ' ' || s.wake_time) AS wake_dt,
        m.latency_min,
        m.awakenings,
        m.quality,
        m.duration_min
    FROM sleep_sessions s
    JOIN sleep_metrics m ON m.session_id = s.id
) base
GROUP BY base.wake_date;

