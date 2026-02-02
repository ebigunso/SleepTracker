use crate::{
    db::Db,
    error::ApiError,
    models::{ExerciseInput, NoteInput, SleepInput, SleepSession},
    repository,
};
use chrono_tz::Tz;
use std::str::FromStr;
fn is_overlap_db_error(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db_err) => db_err
            .message()
            .contains("sleep session overlaps existing session"),
        _ => false,
    }
}

pub async fn create_sleep(db: &Db, input: SleepInput) -> Result<i64, ApiError> {
    input.validate()?;
    let (bed_dt, wake_dt) =
        crate::time::sleep_window_bounds(input.date, input.bed_time, input.wake_time)?;
    let tz = repository::get_user_timezone(db).await;
    let duration =
        crate::time::compute_duration_min(input.date, input.bed_time, input.wake_time, tz)?;
    if repository::has_sleep_overlap(db, bed_dt, wake_dt, None).await? {
        return Err(ApiError::InvalidInput(
            "sleep session overlaps existing session".into(),
        ));
    }
    match repository::insert_sleep(db, &input, duration).await {
        Ok(id) => Ok(id),
        Err(e) if is_overlap_db_error(&e) => Err(ApiError::InvalidInput(
            "sleep session overlaps existing session".into(),
        )),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_sleep_by_date(
    db: &Db,
    date: chrono::NaiveDate,
) -> Result<Vec<SleepSession>, ApiError> {
    Ok(repository::find_sleep_by_date(db, date).await?)
}

pub async fn update_sleep(db: &Db, id: i64, input: SleepInput) -> Result<(), ApiError> {
    input.validate()?;
    let (bed_dt, wake_dt) =
        crate::time::sleep_window_bounds(input.date, input.bed_time, input.wake_time)?;
    let tz = repository::get_user_timezone(db).await;
    let duration =
        crate::time::compute_duration_min(input.date, input.bed_time, input.wake_time, tz)?;
    if repository::has_sleep_overlap(db, bed_dt, wake_dt, Some(id)).await? {
        return Err(ApiError::InvalidInput(
            "sleep session overlaps existing session".into(),
        ));
    }
    let updated = match repository::update_sleep(db, id, &input, duration).await {
        Ok(updated) => updated,
        Err(e) if is_overlap_db_error(&e) => {
            return Err(ApiError::InvalidInput(
                "sleep session overlaps existing session".into(),
            ));
        }
        Err(e) => return Err(e.into()),
    };
    if !updated {
        return Err(ApiError::NotFound);
    }
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

pub async fn set_user_timezone(db: &Db, timezone: String) -> Result<(), ApiError> {
    let tz = Tz::from_str(timezone.trim())
        .map_err(|_| ApiError::InvalidInput("invalid timezone".into()))?;
    repository::set_user_timezone(db, tz.name()).await?;
    Ok(())
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
        let fetched = get_sleep_by_date(&db, input.date).await.unwrap();
        assert_eq!(fetched.len(), 1);
        assert_eq!(fetched[0].id, id);
        assert_eq!(fetched[0].bed_time, input.bed_time);
    }
}
