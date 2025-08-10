#![doc = r#"Domain model and error types

Contains the error type used for validating inputs and enforcing invariants across
the crate. Errors from this module are propagated by many functions using the `?`
operator. See [`time::compute_duration_min`] and [`models::sleep::SleepInput::validate`].

[`time::compute_duration_min`]: crate::time::compute_duration_min
[`models::sleep::SleepInput::validate`]: crate::models::sleep::SleepInput::validate
"#]

use thiserror::Error;

#[doc = r#"Domain-level error type.

Variants:
- `InvalidIntensity(String)`: Parsing or validation failure for exercise intensity.
- `InvalidQuality`: Sleep quality must be in the 1..=5 range.
- `InvalidInput(String)`: Generic validation failure, e.g. invalid ranges or non-positive duration.

# Example (propagating with ?)

```rust
# use chrono::{NaiveDate, NaiveTime};
# use chrono_tz::Asia::Tokyo;
# fn main() -> Result<(), sleep_api::domain::DomainError> {
let mins = sleep_api::time::compute_duration_min(
    NaiveDate::from_ymd_opt(2025, 6, 1).ok_or_else(|| sleep_api::domain::DomainError::InvalidInput("invalid date".into()))?,
    NaiveTime::from_hms_opt(22, 30, 0).ok_or_else(|| sleep_api::domain::DomainError::InvalidInput("invalid time".into()))?,
    NaiveTime::from_hms_opt(6, 30, 0).ok_or_else(|| sleep_api::domain::DomainError::InvalidInput("invalid time".into()))?,
    Tokyo,
)?;
assert!(mins > 0);
# Ok(()) }
```
"#]
#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum DomainError {
    #[error("invalid intensity: {0}")]
    InvalidIntensity(String),
    #[error("quality must be between 1 and 5")]
    InvalidQuality,
    #[error("{0}")]
    InvalidInput(String),
}
