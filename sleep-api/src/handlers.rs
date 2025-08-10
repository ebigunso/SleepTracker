use crate::{
    db::Db,
    error::ApiError,
    models::{ExerciseInput, NoteInput, SleepInput, SleepSession},
    repository,
};

pub async fn create_sleep(db: &Db, input: SleepInput) -> Result<i64, ApiError> {
    input.validate()?;
    let tz = crate::config::app_tz();
    let duration = crate::time::compute_duration_min(input.date, input.bed_time, input.wake_time, tz)?;
    Ok(repository::insert_sleep(db, &input, duration).await?)
}

pub async fn get_sleep_by_date(
    db: &Db,
    date: chrono::NaiveDate,
) -> Result<Option<SleepSession>, ApiError> {
    Ok(repository::find_sleep_by_date(db, date).await?)
}

pub async fn update_sleep(db: &Db, id: i64, input: SleepInput) -> Result<(), ApiError> {
    input.validate()?;
    let tz = crate::config::app_tz();
    let duration = crate::time::compute_duration_min(input.date, input.bed_time, input.wake_time, tz)?;
    repository::update_sleep(db, id, &input, duration).await?;
    Ok(())
}

pub async fn delete_sleep(db: &Db, id: i64) -> Result<u64, ApiError> {
    repository::delete_sleep(db, id).await.map_err(Into::into)
}

pub async fn create_exercise(db: &Db, input: ExerciseInput) -> Result<i64, ApiError> {
    input.validate()?;
    Ok(repository::insert_exercise(db, &input).await?)
}

pub async fn create_note(db: &Db, input: NoteInput) -> Result<i64, ApiError> {
    input.validate()?;
    Ok(repository::insert_note(db, &input).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Quality;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup() -> Db {
        let db = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
            .await
            .unwrap()
            .run(&db)
            .await
            .unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_and_get_sleep() {
        let db = setup().await;
        let input = SleepInput {
            date: chrono::NaiveDate::from_ymd_opt(2025, 6, 17).unwrap(),
            bed_time: chrono::NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            wake_time: chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
            latency_min: 10,
            awakenings: 1,
            quality: Quality(4),
        };
        let id = create_sleep(&db, input.clone()).await.unwrap();
        let fetched = get_sleep_by_date(&db, input.date).await.unwrap().unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.bed_time, input.bed_time);
    }
}
