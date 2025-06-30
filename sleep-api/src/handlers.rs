use crate::{
    db::Db,
    error::ApiError,
    models::{ExerciseInput, NoteInput, SleepInput, SleepSession},
};

pub async fn create_sleep(db: &Db, input: SleepInput) -> Result<i64, ApiError> {
    sqlx::query("INSERT OR IGNORE INTO days(date) VALUES (?)")
        .bind(input.date)
        .execute(db)
        .await?;
    let res = sqlx::query("INSERT INTO sleep_sessions(date, bed_time, wake_time) VALUES (?, ?, ?)")
        .bind(input.date)
        .bind(input.bed_time)
        .bind(input.wake_time)
        .execute(db)
        .await?;
    let id = res.last_insert_rowid();
    sqlx::query("INSERT INTO sleep_metrics(session_id, latency_min, awakenings, quality) VALUES (?, ?, ?, ?)")
        .bind(id)
        .bind(input.latency_min)
        .bind(input.awakenings)
        .bind(input.quality)
        .execute(db)
        .await?;
    Ok(id)
}

pub async fn get_sleep_by_date(
    db: &Db,
    date: chrono::NaiveDate,
) -> Result<Option<SleepSession>, ApiError> {
    let row = sqlx::query_as::<_, SleepSession>(
        r#"SELECT s.id, s.date, s.bed_time, s.wake_time, m.latency_min, m.awakenings, m.quality
           FROM sleep_sessions s JOIN sleep_metrics m ON m.session_id = s.id WHERE s.date = ?"#,
    )
    .bind(date)
    .fetch_optional(db)
    .await?;
    Ok(row)
}

pub async fn update_sleep(db: &Db, id: i64, input: SleepInput) -> Result<(), ApiError> {
    sqlx::query("INSERT OR IGNORE INTO days(date) VALUES (?)")
        .bind(input.date)
        .execute(db)
        .await?;
    sqlx::query("UPDATE sleep_sessions SET date=?, bed_time=?, wake_time=? WHERE id=?")
        .bind(input.date)
        .bind(input.bed_time)
        .bind(input.wake_time)
        .bind(id)
        .execute(db)
        .await?;
    sqlx::query(
        "UPDATE sleep_metrics SET latency_min=?, awakenings=?, quality=? WHERE session_id=?",
    )
    .bind(input.latency_min)
    .bind(input.awakenings)
    .bind(input.quality)
    .bind(id)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn delete_sleep(db: &Db, id: i64) -> Result<u64, ApiError> {
    let res = sqlx::query("DELETE FROM sleep_sessions WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(res.rows_affected())
}

pub async fn create_exercise(db: &Db, input: ExerciseInput) -> Result<i64, ApiError> {
    sqlx::query("INSERT OR IGNORE INTO days(date) VALUES (?)")
        .bind(input.date)
        .execute(db)
        .await?;
    let res = sqlx::query("INSERT INTO exercise_events(date, intensity, start_time, duration_min) VALUES (?, ?, ?, ?)")
        .bind(input.date)
        .bind(input.intensity)
        .bind(input.start_time)
        .bind(input.duration_min)
        .execute(db)
        .await?;
    Ok(res.last_insert_rowid())
}

pub async fn create_note(db: &Db, input: NoteInput) -> Result<i64, ApiError> {
    sqlx::query("INSERT OR IGNORE INTO days(date) VALUES (?)")
        .bind(input.date)
        .execute(db)
        .await?;
    let res = sqlx::query("INSERT INTO notes(date, body) VALUES (?, ?)")
        .bind(input.date)
        .bind(input.body)
        .execute(db)
        .await?;
    Ok(res.last_insert_rowid())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup() -> Db {
        let db = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("../migrations").run(&db).await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_and_get_sleep() {
        let db = setup().await;
        let input = SleepInput {
            date: chrono::NaiveDate::from_ymd_opt(2025, 6, 17).unwrap(),
            bed_time: chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
            wake_time: chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            latency_min: 10,
            awakenings: 1,
            quality: 4,
        };
        let id = create_sleep(&db, input.clone()).await.unwrap();
        let fetched = get_sleep_by_date(&db, input.date).await.unwrap().unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.bed_time, input.bed_time);
    }
}
