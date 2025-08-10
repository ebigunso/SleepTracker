-- Add duration_min to sleep_metrics and a partial unique index for daily exercise sentinel rows
ALTER TABLE sleep_metrics ADD COLUMN duration_min INTEGER NOT NULL DEFAULT 0;

-- Ensure at most one "daily intensity" sentinel row per date
CREATE UNIQUE INDEX IF NOT EXISTS daily_exercise_unique
  ON exercise_events(date)
  WHERE start_time IS NULL AND duration_min IS NULL;
