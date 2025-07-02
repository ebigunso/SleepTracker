use chrono::NaiveDate;
use sqlx::{Sqlite, Transaction};
use crate::{db::Db, models::{SleepInput, SleepSession, ExerciseInput, NoteInput}};

pub async fn insert_sleep(db: &Db, input: &SleepInput) -> Result<i64, sqlx::Error> {
    let mut tx: Transaction<'_, Sqlite> = db.begin().await?;
    let res = sqlx::query::<Sqlite>(
        "INSERT INTO sleep_sessions(date, bed_time, wake_time) VALUES (?, ?, ?)"
    )
    .bind(input.date)
    .bind(input.bed_time)
    .bind(input.wake_time)
    .execute(&mut *tx)
    .await?;
    let id = res.last_insert_rowid();
    sqlx::query::<Sqlite>(
        "INSERT INTO sleep_metrics(session_id, latency_min, awakenings, quality) VALUES (?, ?, ?, ?)"
    )
    .bind(id)
    .bind(input.latency_min)
    .bind(input.awakenings)
    .bind(input.quality.value() as i32)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(id)
}

pub async fn find_sleep_by_date(db: &Db, date: NaiveDate) -> Result<Option<SleepSession>, sqlx::Error> {
    sqlx::query_as::<Sqlite, SleepSession>(
        r#"SELECT s.id, s.date, s.bed_time, s.wake_time, m.latency_min, m.awakenings, m.quality
           FROM sleep_sessions s JOIN sleep_metrics m ON m.session_id = s.id WHERE s.date = ?"#,
    )
    .bind(date)
    .fetch_optional(db)
    .await
}

pub async fn update_sleep(db: &Db, id: i64, input: &SleepInput) -> Result<(), sqlx::Error> {
    let mut tx: Transaction<'_, Sqlite> = db.begin().await?;
    sqlx::query::<Sqlite>("UPDATE sleep_sessions SET date=?, bed_time=?, wake_time=? WHERE id=?")
        .bind(input.date)
        .bind(input.bed_time)
        .bind(input.wake_time)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    sqlx::query::<Sqlite>("UPDATE sleep_metrics SET latency_min=?, awakenings=?, quality=? WHERE session_id=?")
        .bind(input.latency_min)
        .bind(input.awakenings)
        .bind(input.quality.value() as i32)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn delete_sleep(db: &Db, id: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query::<Sqlite>("DELETE FROM sleep_sessions WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(res.rows_affected())
}

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

pub async fn insert_note(db: &Db, input: &NoteInput) -> Result<i64, sqlx::Error> {
    let res = sqlx::query::<Sqlite>("INSERT INTO notes(date, body) VALUES (?, ?)")
        .bind(input.date)
        .bind(input.body.as_deref())
        .execute(db)
        .await?;
    Ok(res.last_insert_rowid())
}
