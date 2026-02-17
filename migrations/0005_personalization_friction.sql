-- Personalization friction telemetry (append-only)
-- Captures per-submit workflow friction signals for rolling-window analysis.

CREATE TABLE IF NOT EXISTS personalization_friction_events (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    recorded_at         DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    form_time_ms        INTEGER NOT NULL CHECK (form_time_ms >= 0),
    error_kind          TEXT,
    retry_count         INTEGER NOT NULL DEFAULT 0 CHECK (retry_count >= 0),
    immediate_edit      INTEGER NOT NULL DEFAULT 0 CHECK (immediate_edit IN (0, 1)),
    follow_up_failure   INTEGER NOT NULL DEFAULT 0 CHECK (follow_up_failure IN (0, 1))
);

-- Append-only guardrails
CREATE TRIGGER IF NOT EXISTS personalization_friction_events_no_update
BEFORE UPDATE ON personalization_friction_events
FOR EACH ROW
BEGIN
    SELECT RAISE(ABORT, 'personalization_friction_events is append-only');
END;

CREATE TRIGGER IF NOT EXISTS personalization_friction_events_no_delete
BEFORE DELETE ON personalization_friction_events
FOR EACH ROW
BEGIN
    SELECT RAISE(ABORT, 'personalization_friction_events is append-only');
END;

-- Rolling-window query performance indexes
CREATE INDEX IF NOT EXISTS idx_friction_events_recorded_at
    ON personalization_friction_events(recorded_at);

CREATE INDEX IF NOT EXISTS idx_friction_events_error_kind_recorded_at
    ON personalization_friction_events(error_kind, recorded_at);

CREATE INDEX IF NOT EXISTS idx_friction_events_immediate_edit_recorded_at
    ON personalization_friction_events(recorded_at)
    WHERE immediate_edit = 1;

CREATE INDEX IF NOT EXISTS idx_friction_events_follow_up_failure_recorded_at
    ON personalization_friction_events(recorded_at)
    WHERE follow_up_failure = 1;
