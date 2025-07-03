use super::intensity::Intensity;
use crate::domain::DomainError;
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ExerciseInput {
    pub date: NaiveDate,
    pub intensity: Intensity,
    pub start_time: Option<NaiveTime>,
    pub duration_min: Option<i32>,
}

impl ExerciseInput {
    pub fn validate(&self) -> Result<(), DomainError> {
        // intensity is validated by deserialization
        Ok(())
    }
}
