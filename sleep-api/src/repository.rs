#![doc = r#"Persistence layer

Wraps SQLx statements for inserting, updating, fetching, and deleting domain records.
Operations that touch multiple tables use a single transaction to keep data consistent.

Why: prefer using these helpers over ad-hoc queries to ensure invariants and transactional correctness.

See also:
- [`models`] for data shapes
- [`time::compute_duration_min`] for deriving duration values
- Examples on [`insert_sleep`] demonstrating end-to-end usage.

[`models`]: crate::models
[`time::compute_duration_min`]: crate::time::compute_duration_min
[`insert_sleep`]: crate::repository::insert_sleep
"#]

use crate::{
    db::Db,
    models::{ExerciseInput, NoteInput, SleepInput, SleepListItem, SleepSession, DateIntensity},
};
use chrono::NaiveDate;
use sqlx::{Sqlite, Transaction};

#[doc = r#"Insert a sleep session and its metrics in a single transaction.

The session row is written to `sleep_sessions` and the metrics to `sleep_metrics`.
Pass a precomputed `duration_min` (see [`time::compute_duration_min`]).

# Example

```rust,no_run
# use sleep_api::domain::DomainError;
# use std::error::Error;
# use sleep_api::{db, repository, models::{SleepInput, Quality}};
# use chrono::{NaiveDate, NaiveTime};
# async fn demo() -> Result<(), Box<dyn Error>> {
// Ensure DATABASE_URL is set in the environment (e.g., sqlite::memory:).
let db = db::connect().await?;
sqlx::migrate::Migrator::new(std::path::Path::new("../migrations")).await?.run(&db).await?;

let input = SleepInput {
    date: NaiveDate::from_ymd_opt(2025, 6, 1).ok_or_else(|| DomainError::InvalidInput("invalid date".into()))?,
    bed_time: NaiveTime::from_hms_opt(23, 0, 0).ok_or_else(|| DomainError::InvalidInput("invalid time".into()))?,
    wake_time: NaiveTime::from_hms_opt(7, 0, 0).ok_or_else(|| DomainError::InvalidInput("invalid time".into()))?,
    latency_min: 10,
    awakenings: 1,
    quality: Quality(4),
};
let tz = sleep_api::config::app_tz();
let dur = sleep_api::time::compute_duration_min(input.date, input.bed_time, input.wake_time, tz)?;
let id = repository::insert_sleep(&db, &input, dur).await?;
# Ok(()) }
```

# Errors
- Returns [`sqlx::Error`] on database connection or execution errors.

[`time::compute_duration_min`]: crate::time::compute_duration_min
"#]
pub async fn insert_sleep(
    db: &Db,
    input: &SleepInput,
    duration_min: i32,
) -> Result<i64, sqlx::Error> {
    let mut tx: Transaction<'_, Sqlite> = db.begin().await?;
    let res = sqlx::query::<Sqlite>(
        "INSERT INTO sleep_sessions(date, bed_time, wake_time) VALUES (?, ?, ?)",
    )
    .bind(input.date)
    .bind(input.bed_time)
    .bind(input.wake_time)
    .execute(&mut *tx)
    .await?;
    let id = res.last_insert_rowid();
    sqlx::query::<Sqlite>(
        "INSERT INTO sleep_metrics(session_id, latency_min, awakenings, quality, duration_min) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(id)
    .bind(input.latency_min)
    .bind(input.awakenings)
    .bind(input.quality.value() as i32)
    .bind(duration_min)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(id)
}

#[doc = r#"Find a sleep session by wake date.

Returns `Ok(None)` if no session exists for the provided date.

See the example on [`insert_sleep`].

# Errors
- Returns [`sqlx::Error`] on database errors.
"#]
pub async fn find_sleep_by_date(
    db: &Db,
    date: NaiveDate,
) -> Result<Option<SleepSession>, sqlx::Error> {
    sqlx::query_as::<Sqlite, SleepSession>(
        r#"SELECT s.id, s.date, s.bed_time, s.wake_time, m.latency_min, m.awakenings, m.quality
           FROM sleep_sessions s JOIN sleep_metrics m ON m.session_id = s.id WHERE s.date = ?"#,
    )
    .bind(date)
    .fetch_optional(db)
    .await
}

#[doc = r#"Find a sleep session by id.

Returns `Ok(None)` if no session exists for the provided id.

See the example on [`insert_sleep`].

# Errors
- Returns [`sqlx::Error`] on database errors.
"#]
pub async fn find_sleep_by_id(db: &Db, id: i64) -> Result<Option<SleepSession>, sqlx::Error> {
    sqlx::query_as::<Sqlite, SleepSession>(
        r#"SELECT s.id, s.date, s.bed_time, s.wake_time, m.latency_min, m.awakenings, m.quality
           FROM sleep_sessions s JOIN sleep_metrics m ON m.session_id = s.id WHERE s.id = ?"#,
    )
    .bind(id)
    .fetch_optional(db)
    .await
}

#[doc = r#"Update a sleep session and its metrics in a single transaction.

Requires a recomputed `duration_min`; see [`time::compute_duration_min`].
See the example on [`insert_sleep`].

# Errors
- Returns [`sqlx::Error`] on database errors.
"#]
pub async fn update_sleep(
    db: &Db,
    id: i64,
    input: &SleepInput,
    duration_min: i32,
) -> Result<(), sqlx::Error> {
    let mut tx: Transaction<'_, Sqlite> = db.begin().await?;
    sqlx::query::<Sqlite>("UPDATE sleep_sessions SET date=?, bed_time=?, wake_time=? WHERE id=?")
        .bind(input.date)
        .bind(input.bed_time)
        .bind(input.wake_time)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    sqlx::query::<Sqlite>(
        "UPDATE sleep_metrics SET latency_min=?, awakenings=?, quality=?, duration_min=? WHERE session_id=?",
    )
    .bind(input.latency_min)
    .bind(input.awakenings)
    .bind(input.quality.value() as i32)
    .bind(duration_min)
    .bind(id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

#[doc = r#"Delete a sleep session by id.

Returns the number of rows affected (0 if no such id exists).

See the example on [`insert_sleep`].

# Errors
- Returns [`sqlx::Error`] on database errors.
"#]
pub async fn delete_sleep(db: &Db, id: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query::<Sqlite>("DELETE FROM sleep_sessions WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(res.rows_affected())
}

#[doc = r#"List last N daily sleep entries ordered by date DESC.

Backed by the v_daily_sleep view. Maps wake_date -> date via SQL alias to match API struct."#]
pub async fn list_recent_sleep(db: &Db, days: i32) -> Result<Vec<SleepListItem>, sqlx::Error> {
    sqlx::query_as::<Sqlite, SleepListItem>(
        r#"SELECT id,
                   wake_date AS date,
                   bed_time,
                   wake_time,
                   latency_min,
                   awakenings,
                   quality,
                   duration_min
          FROM v_daily_sleep
          ORDER BY date DESC
          LIMIT ?"#,
    )
    .bind(days)
    .fetch_all(db)
    .await
}

#[doc = r#"List exercise intensity by date in the inclusive range [from, to].

For each date, returns the highest intensity among any events on that date.

- "none" < "light" < "hard"

Ordered by date ASC.
"#]
pub async fn list_exercise_intensity(
    db: &Db,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<DateIntensity>, sqlx::Error> {
    // Map intensity to ordinal to pick max, then map back to string
    sqlx::query_as::<Sqlite, DateIntensity>(
        r#"
        SELECT
          date,
          CASE MAX(CASE intensity WHEN 'none' THEN 0 WHEN 'light' THEN 1 WHEN 'hard' THEN 2 ELSE 0 END)
            WHEN 2 THEN 'hard'
            WHEN 1 THEN 'light'
            ELSE 'none'
          END AS intensity
        FROM exercise_events
        WHERE date BETWEEN ? AND ?
        GROUP BY date
        ORDER BY date ASC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(db)
}

#[doc = r#"List daily sleep entries in the inclusive range [from, to] ordered by date ASC."#]
pub async fn list_sleep_range(
    db: &Db,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<SleepListItem>, sqlx::Error> {
    sqlx::query_as::<Sqlite, SleepListItem>(
        r#"SELECT id,
                   wake_date AS date,
                   bed_time,
                   wake_time,
                   latency_min,
                   awakenings,
                   quality,
                   duration_min
          FROM v_daily_sleep
          WHERE wake_date BETWEEN ? AND ?
          ORDER BY date ASC"#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await
}

#[doc = r#"Insert an exercise event.

# Example (minimal)

```rust,no_run
# use sleep_api::domain::DomainError;
# use std::error::Error;
# use sleep_api::{db, repository, models::{ExerciseInput, Intensity}};
# use chrono::NaiveDate;
# async fn demo() -> Result<(), Box<dyn Error>> {
// Ensure DATABASE_URL is set in the environment (e.g., sqlite::memory:).
let db = db::connect().await?;
sqlx::migrate::Migrator::new(std::path::Path::new("../migrations")).await?.run(&db).await?;

let input = ExerciseInput {
    date: NaiveDate::from_ymd_opt(2025, 6, 1).ok_or_else(|| DomainError::InvalidInput("invalid date".into()))?,
    intensity: Intensity::Light,
    start_time: None,
    duration_min: Some(30),
};
input.validate()?;
let id = repository::insert_exercise(&db, &input).await?;
# Ok(()) }
```

# Errors
- Returns [`sqlx::Error`] on database errors.
"#]
pub async fn insert_exercise(db: &Db, input: &ExerciseInput) -> Result<i64, sqlx::Error> {
    let mut tx: Transaction<'_, Sqlite> = db.begin().await?;
    let res = sqlx::query::<Sqlite>(
        "INSERT INTO exercise_events(date, intensity, start_time, duration_min) VALUES (?, ?, ?, ?)"
    )
    .bind(input.date)
    .bind(input.intensity.to_string())
    .bind(input.start_time)
    .bind(input.duration_min)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(res.last_insert_rowid())
}

#[doc = r#"Insert a note for a particular date.

A `None` body is stored as NULL.

# Example

```rust,no_run
# use sleep_api::domain::DomainError;
# use std::error::Error;
# use sleep_api::{db, repository, models::NoteInput};
# use chrono::NaiveDate;
# async fn demo() -> Result<(), Box<dyn Error>> {
// Ensure DATABASE_URL is set in the environment (e.g., sqlite::memory:).
let db = db::connect().await?;
sqlx::migrate::Migrator::new(std::path::Path::new("../migrations")).await?.run(&db).await?;

let input = NoteInput {
    date: NaiveDate::from_ymd_opt(2025, 6, 1).ok_or_else(|| DomainError::InvalidInput("invalid date".into()))?,
    body: Some("Slept well".to_string()),
};
input.validate()?;
let id = repository::insert_note(&db, &input).await?;
# Ok(()) }
```

# Errors
- Returns [`sqlx::Error`] on database errors.
"#]
pub async fn insert_note(db: &Db, input: &NoteInput) -> Result<i64, sqlx::Error> {
    let res = sqlx::query::<Sqlite>("INSERT INTO notes(date, body) VALUES (?, ?)")
        .bind(input.date)
        .bind(input.body.as_deref())
        .execute(db)
        .await?;
    Ok(res.last_insert_rowid())
}
