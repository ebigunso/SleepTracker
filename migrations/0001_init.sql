CREATE TABLE sleep_sessions (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    date            DATE NOT NULL,
    bed_time        TIME NOT NULL,
    wake_time       TIME NOT NULL
);

CREATE TABLE sleep_metrics (
    session_id      INTEGER PRIMARY KEY REFERENCES sleep_sessions(id) ON DELETE CASCADE,
    latency_min     INTEGER NOT NULL,
    awakenings      INTEGER NOT NULL,
    quality         INTEGER NOT NULL CHECK (quality BETWEEN 1 AND 5)
);

CREATE TABLE exercise_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    date            DATE NOT NULL,
    intensity       TEXT NOT NULL CHECK (intensity IN ('none','light','hard')),
    start_time      TIME,
    duration_min    INTEGER
);

CREATE TABLE notes (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    date            DATE NOT NULL,
    body            TEXT
);
