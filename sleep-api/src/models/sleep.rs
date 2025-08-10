use super::quality::Quality;
use crate::domain::DomainError;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone)]
pub struct SleepInput {
    pub date: NaiveDate,
    pub bed_time: NaiveTime,
    pub wake_time: NaiveTime,
    pub latency_min: i32,
    pub awakenings: i32,
    pub quality: Quality,
}

impl SleepInput {
    pub fn validate(&self) -> Result<(), DomainError> {
        if !(0..=180).contains(&self.latency_min) {
            return Err(DomainError::InvalidInput(
                "latency_min must be between 0 and 180".into(),
            ));
        }
        if !(0..=10).contains(&self.awakenings) {
            return Err(DomainError::InvalidInput(
                "awakenings must be between 0 and 10".into(),
            ));
        }
        // quality validated by type; time relationship validated via duration computation in handlers
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow)]
pub struct SleepSession {
    pub id: i64,
    pub date: NaiveDate,
    pub bed_time: NaiveTime,
    pub wake_time: NaiveTime,
    pub latency_min: i32,
    pub awakenings: i32,
    pub quality: i32,
}
