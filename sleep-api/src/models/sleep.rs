use super::quality::Quality;
use crate::domain::DomainError;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[doc = r#"User-provided input for creating or updating a sleep session.

Field semantics (wake-date model):
- `date`: the wake date of the sleep (the morning date).
- `bed_time` / `wake_time`: local times. If `bed_time > wake_time`, the bed datetime
  is considered to be on the previous calendar day.
- `latency_min`: minutes to fall asleep, must be in 0..=180.
- `awakenings`: number of awakenings, must be in 0..=10.
- `quality`: discrete quality score enforced by [`Quality`] (1..=5).

For duration computations across DST, see [`compute_duration_min`].

# Example

```rust
# use sleep_api::domain::DomainError;
# use sleep_api::models::{SleepInput, Quality};
# use chrono::{NaiveDate, NaiveTime};
# fn main() -> Result<(), DomainError> {
let input = SleepInput {
    date: NaiveDate::from_ymd_opt(2025, 6, 1).ok_or_else(|| DomainError::InvalidInput("invalid date".into()))?,
    bed_time: NaiveTime::from_hms_opt(23, 0, 0).ok_or_else(|| DomainError::InvalidInput("invalid time".into()))?,
    wake_time: NaiveTime::from_hms_opt(7, 0, 0).ok_or_else(|| DomainError::InvalidInput("invalid time".into()))?,
    latency_min: 10,
    awakenings: 1,
    quality: Quality(4),
};
input.validate()?;
# Ok(()) }
```

[`compute_duration_min`]: crate::time::compute_duration_min
[`Quality`]: crate::models::Quality
"#]
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
    #[doc = r#"Validate input ranges for latency and awakenings.

- `latency_min` must be in 0..=180
- `awakenings` must be in 0..=10
- `quality` is validated by the [`Quality`] type
- Time relationships are validated at duration computation time (see [`compute_duration_min`]).

# Errors

Returns [`DomainError::InvalidInput`] when a field is out of range.

[`Quality`]: crate::models::Quality
[`compute_duration_min`]: crate::time::compute_duration_min
"#]
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

#[doc = r#"Database projection of a stored sleep session.

This type aggregates fields from `sleep_sessions` and `sleep_metrics` for a given session id.

Note: `quality` is stored as `i32` in the DB layer; use [`Quality::try_from`] to convert into the strong type if needed.

[`Quality::try_from`]: crate::models::Quality::try_from
"#]
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

#[doc = r#"List item projection for v_daily_sleep.

Used by GET /sleep/recent and GET /sleep/range. Note that the SQL maps
`wake_date` to `date` via `AS date` to align with the existing field name.
`duration_min` is nullable (computed on insert/update; may be NULL for legacy rows).

Fields mirror v_daily_sleep columns:
- id
- date (wake date)
- bed_time
- wake_time
- latency_min
- awakenings
- quality
- duration_min (nullable)
"#]
#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow, Clone)]
pub struct SleepListItem {
    pub id: i64,
    pub date: NaiveDate,
    pub bed_time: NaiveTime,
    pub wake_time: NaiveTime,
    pub latency_min: i32,
    pub awakenings: i32,
    pub quality: i32,
    pub duration_min: Option<i32>,
}
