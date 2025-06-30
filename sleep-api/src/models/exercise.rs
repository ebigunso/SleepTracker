use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ExerciseInput {
    pub date: NaiveDate,
    pub intensity: String,
    pub start_time: Option<NaiveTime>,
    pub duration_min: Option<i32>,
}
