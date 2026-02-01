use super::intensity::Intensity;
use crate::domain::DomainError;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[doc = r#"User-provided input representing an exercise event.

Fields:
- `date`: calendar date of the exercise.
- `intensity`: qualitative intensity level, see [`Intensity`].
- `start_time`: optional local start time.
- `duration_min`: optional duration in minutes.

# Example

```rust
# use sleep_api::domain::DomainError;
# use sleep_api::models::{ExerciseInput, Intensity};
# use chrono::{NaiveDate, NaiveTime};
# fn main() -> Result<(), DomainError> {
let ex = ExerciseInput {
    date: NaiveDate::from_ymd_opt(2025, 6, 1).ok_or_else(|| DomainError::InvalidInput("invalid date".into()))?,
    intensity: Intensity::Light,
    start_time: Some(NaiveTime::from_hms_opt(18, 30, 0).ok_or_else(|| DomainError::InvalidInput("invalid time".into()))?),
    duration_min: Some(45),
};
ex.validate()?;
# Ok(()) }
```

[`Intensity`]: crate::models::Intensity
"#]
#[derive(Serialize, Deserialize, Clone)]
pub struct ExerciseInput {
    pub date: NaiveDate,
    pub intensity: Intensity,
    pub start_time: Option<NaiveTime>,
    pub duration_min: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow, Clone)]
pub struct DateIntensity {
    pub date: NaiveDate,
    pub intensity: String, // "none" | "light" | "hard"
}

impl ExerciseInput {
    #[doc = r#"Validate the exercise input.

Currently, this ensures that `intensity` has been deserialized into a valid value.
Duration (when provided) must be in 1..=1440 minutes.

# Errors

Returns [`DomainError`] if a validation rule is violated.
"#]
    pub fn validate(&self) -> Result<(), DomainError> {
        if let Some(duration_min) = self.duration_min {
            if !(1..=MAX_EXERCISE_DURATION_MIN).contains(&duration_min) {
                return Err(DomainError::InvalidInput(format!(
                    "duration_min must be between 1 and {MAX_EXERCISE_DURATION_MIN}"
                )));
            }
        }
        // intensity is validated by deserialization
        Ok(())
    }
}

const MAX_EXERCISE_DURATION_MIN: i32 = 24 * 60;
