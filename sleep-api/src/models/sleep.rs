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
    pub quality: i32,
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
