use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone)]
pub struct FrictionTelemetryInput {
    pub form_time_ms: i32,
    pub error_kind: Option<String>,
    pub retry_count: i32,
    pub immediate_edit: bool,
    pub follow_up_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow, Clone)]
pub struct FrictionTelemetryEvent {
    pub id: i64,
    pub recorded_at: NaiveDateTime,
    pub form_time_ms: i32,
    pub error_kind: Option<String>,
    pub retry_count: i32,
    pub immediate_edit: bool,
    pub follow_up_failure: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow, Clone)]
pub struct FrictionWindowAggregate {
    pub submit_count: i64,
    pub median_form_time_ms: f64,
    pub avg_form_time_ms: f64,
    pub error_count: i64,
    pub retries_total: i64,
    pub retries_avg: f64,
    pub immediate_edit_count: i64,
    pub follow_up_failure_count: i64,
    pub error_rate: f64,
    pub immediate_edit_rate: f64,
    pub follow_up_failure_rate: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow, Clone)]
pub struct FrictionErrorKindAggregate {
    pub error_kind: String,
    pub occurrences: i64,
    pub retries_total: i64,
    pub avg_form_time_ms: f64,
    pub immediate_edit_count: i64,
    pub follow_up_failure_count: i64,
}
