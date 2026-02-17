use crate::{
    db::Db,
    error::ApiError,
    models::{ExerciseInput, FrictionTelemetryInput, NoteInput, SleepInput, SleepSession},
    repository,
};
use chrono::{Duration as ChronoDuration, NaiveDate, NaiveDateTime, Utc};
use chrono_tz::Tz;
use serde::Serialize;
use std::collections::HashMap;
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

pub async fn get_user_timezone(db: &Db) -> String {
    let tz = repository::get_user_timezone(db).await;
    tz.name().to_string()
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FrictionProposalConfidence {
    High,
    Medium,
    Low,
}

impl FrictionProposalConfidence {
    fn at_least_medium(&self) -> bool {
        matches!(
            self,
            FrictionProposalConfidence::High | FrictionProposalConfidence::Medium
        )
    }
}

#[derive(Serialize, Clone)]
pub struct FrictionProposalEvidence {
    pub current_occurrences: i64,
    pub prior_occurrences: i64,
    pub current_submit_count: i64,
    pub prior_submit_count: i64,
    pub current_avg_form_time_ms: f64,
    pub prior_avg_form_time_ms: f64,
    pub current_retry_avg: f64,
    pub prior_retry_avg: f64,
    pub current_follow_up_failure_rate: f64,
    pub prior_follow_up_failure_rate: f64,
}

#[derive(Serialize, Clone)]
pub struct FrictionBacklogProposal {
    pub rank: usize,
    pub action_key: String,
    pub observed_evidence: FrictionProposalEvidence,
    pub expected_benefit: String,
    pub estimated_minutes_saved_per_week: f64,
    pub confidence: FrictionProposalConfidence,
    pub persistence_two_windows: bool,
    pub rollback_condition: String,
    pub auto_promoted: bool,
}

#[derive(Serialize, Clone)]
pub struct FrictionBacklogWindow {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub submit_count: i64,
}

#[derive(Serialize, Clone)]
pub struct FrictionBacklogResponse {
    pub as_of: NaiveDate,
    pub window_days: i64,
    pub minimum_sample_met: bool,
    pub current_window: FrictionBacklogWindow,
    pub prior_window: FrictionBacklogWindow,
    pub proposals: Vec<FrictionBacklogProposal>,
}

fn validate_friction_input(input: &FrictionTelemetryInput) -> Result<(), ApiError> {
    if input.form_time_ms < 0 {
        return Err(ApiError::InvalidInput(
            "form_time_ms must be >= 0".to_string(),
        ));
    }
    if input.retry_count < 0 {
        return Err(ApiError::InvalidInput(
            "retry_count must be >= 0".to_string(),
        ));
    }
    Ok(())
}

pub async fn create_friction_telemetry(
    db: &Db,
    mut input: FrictionTelemetryInput,
) -> Result<i64, ApiError> {
    validate_friction_input(&input)?;
    input.error_kind = input
        .error_kind
        .map(|value| value.trim().to_lowercase())
        .filter(|value| !value.is_empty());
    Ok(repository::insert_friction_telemetry(db, &input).await?)
}

fn start_of_day(date: NaiveDate) -> Result<NaiveDateTime, ApiError> {
    date.and_hms_opt(0, 0, 0)
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))
}

fn end_of_day(date: NaiveDate) -> Result<NaiveDateTime, ApiError> {
    date.and_hms_opt(23, 59, 59)
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))
}

pub async fn friction_backlog(
    db: &Db,
    window_days: i64,
    to: Option<NaiveDate>,
) -> Result<FrictionBacklogResponse, ApiError> {
    if !(1..=365).contains(&window_days) {
        return Err(ApiError::InvalidInput(
            "window_days must be between 1 and 365".into(),
        ));
    }

    let as_of = to.unwrap_or_else(|| Utc::now().date_naive());
    let current_from = as_of
        .checked_sub_signed(ChronoDuration::days(window_days - 1))
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))?;
    let prior_to = current_from
        .pred_opt()
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))?;
    let prior_from = prior_to
        .checked_sub_signed(ChronoDuration::days(window_days - 1))
        .ok_or_else(|| ApiError::InvalidInput("invalid date range".into()))?;

    let current_agg =
        repository::aggregate_friction_window(db, start_of_day(current_from)?, end_of_day(as_of)?)
            .await?;
    let prior_agg =
        repository::aggregate_friction_window(db, start_of_day(prior_from)?, end_of_day(prior_to)?)
            .await?;

    let current_kinds = repository::aggregate_friction_error_kinds_window(
        db,
        start_of_day(current_from)?,
        end_of_day(as_of)?,
    )
    .await?;
    let prior_kinds = repository::aggregate_friction_error_kinds_window(
        db,
        start_of_day(prior_from)?,
        end_of_day(prior_to)?,
    )
    .await?;

    let prior_by_kind: HashMap<String, crate::models::FrictionErrorKindAggregate> = prior_kinds
        .into_iter()
        .map(|item| (item.error_kind.clone(), item))
        .collect();

    let minimum_sample_met = current_agg.submit_count >= 30;
    let mut proposals = Vec::new();
    for current in current_kinds {
        let prior_occurrences = prior_by_kind
            .get(&current.error_kind)
            .map(|p| p.occurrences)
            .unwrap_or(0);
        let persistence_two_windows = current.occurrences > 0 && prior_occurrences > 0;

        let prior_avg_form_time_ms = prior_by_kind
            .get(&current.error_kind)
            .map(|p| p.avg_form_time_ms)
            .unwrap_or(0.0);
        let prior_retry_avg_for_kind = prior_by_kind
            .get(&current.error_kind)
            .map(|p| {
                if p.occurrences > 0 {
                    p.retries_total as f64 / p.occurrences as f64
                } else {
                    0.0
                }
            })
            .unwrap_or(0.0);
        let current_retry_avg_for_kind = if current.occurrences > 0 {
            current.retries_total as f64 / current.occurrences as f64
        } else {
            0.0
        };
        let current_follow_up_failure_rate_for_kind = if current.occurrences > 0 {
            current.follow_up_failure_count as f64 / current.occurrences as f64
        } else {
            0.0
        };
        let prior_follow_up_failure_rate_for_kind = prior_by_kind
            .get(&current.error_kind)
            .map(|p| {
                if p.occurrences > 0 {
                    p.follow_up_failure_count as f64 / p.occurrences as f64
                } else {
                    0.0
                }
            })
            .unwrap_or(0.0);

        let delta_form_minutes =
            ((current.avg_form_time_ms - prior_avg_form_time_ms).max(0.0)) / 60_000.0;
        let delta_retry = (current_retry_avg_for_kind - prior_retry_avg_for_kind).max(0.0);
        let delta_failure_rate = (current_follow_up_failure_rate_for_kind
            - prior_follow_up_failure_rate_for_kind)
            .max(0.0);
        let events_per_week = (current.occurrences as f64) * 7.0 / (window_days as f64);
        let estimated_minutes_saved_per_week = (events_per_week
            * (delta_form_minutes + (delta_retry * 0.75) + (delta_failure_rate * 1.5)))
            .max(0.0);

        let confidence = if minimum_sample_met
            && persistence_two_windows
            && current.occurrences >= 12
            && prior_occurrences >= 12
        {
            FrictionProposalConfidence::High
        } else if minimum_sample_met
            && persistence_two_windows
            && current.occurrences >= 4
            && prior_occurrences >= 4
        {
            FrictionProposalConfidence::Medium
        } else {
            FrictionProposalConfidence::Low
        };

        let auto_promoted = confidence.at_least_medium() && persistence_two_windows;

        proposals.push(FrictionBacklogProposal {
            rank: 0,
            action_key: format!("friction_reduction_{}", current.error_kind.replace(' ', "_")),
            observed_evidence: FrictionProposalEvidence {
                current_occurrences: current.occurrences,
                prior_occurrences,
                current_submit_count: current_agg.submit_count,
                prior_submit_count: prior_agg.submit_count,
                current_avg_form_time_ms: current.avg_form_time_ms,
                prior_avg_form_time_ms,
                current_retry_avg: current_retry_avg_for_kind,
                prior_retry_avg: prior_retry_avg_for_kind,
                current_follow_up_failure_rate: current_follow_up_failure_rate_for_kind,
                prior_follow_up_failure_rate: prior_follow_up_failure_rate_for_kind,
            },
            expected_benefit: format!(
                "Reduce repeated '{}' friction by lowering retries and form time variance",
                current.error_kind
            ),
            estimated_minutes_saved_per_week,
            confidence,
            persistence_two_windows,
            rollback_condition:
                "Downgrade if pattern no longer persists for two windows or confidence falls below medium"
                    .to_string(),
            auto_promoted,
        });
    }

    proposals.sort_by(|a, b| {
        b.estimated_minutes_saved_per_week
            .total_cmp(&a.estimated_minutes_saved_per_week)
    });
    for (index, proposal) in proposals.iter_mut().enumerate() {
        proposal.rank = index + 1;
    }

    Ok(FrictionBacklogResponse {
        as_of,
        window_days,
        minimum_sample_met,
        current_window: FrictionBacklogWindow {
            from: current_from,
            to: as_of,
            submit_count: current_agg.submit_count,
        },
        prior_window: FrictionBacklogWindow {
            from: prior_from,
            to: prior_to,
            submit_count: prior_agg.submit_count,
        },
        proposals,
    })
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
