#![doc = r#"Time handling utilities

Provides DST-aware resolution and helpers for computing sleep durations
using "wake-date" semantics. See [`compute_duration_min`].

[`compute_duration_min`]: crate::time::compute_duration_min
"#]

use crate::domain::DomainError;
use chrono::{
    DateTime, Duration as ChronoDuration, LocalResult, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Utc,
};
use chrono_tz::Tz;

/// Maximum minutes to scan forward to bridge DST "spring forward" gaps
const MAX_DST_GAP_MINUTES: usize = 3 * 60;

/// Resolve a local naive datetime in a timezone to a concrete instant.
///
/// Handling of DST gaps/overlaps:
/// - Ambiguous: pick the earliest instant.
/// - Non-existent (spring-forward gap): advance in 1-minute steps until valid (up to MAX_DST_GAP_MINUTES).
///
/// Fallback: If no valid local time can be resolved, falls back to interpreting the naive datetime as UTC with a warning. This may affect duration calculations across DST transitions.
fn resolve_local(tz: Tz, ndt: NaiveDateTime) -> DateTime<Tz> {
    match tz.from_local_datetime(&ndt) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(earliest, _latest) => earliest,
        LocalResult::None => {
            // advance forward until it becomes valid
            let mut cur = ndt;
            for _ in 0..MAX_DST_GAP_MINUTES {
                match tz.from_local_datetime(&cur) {
                    LocalResult::Single(dt) => return dt,
                    LocalResult::Ambiguous(earliest, _latest) => return earliest,
                    LocalResult::None => {
                        cur += ChronoDuration::minutes(1);
                    }
                }
            }
            // Fallback: interpret as UTC wall time then project to tz (best-effort)
            tracing::warn!(
                "resolve_local fallback: projecting naive datetime as UTC; tz={:?}, ndt={}",
                tz,
                ndt
            );
            tz.from_utc_datetime(&ndt)
        }
    }
}

#[doc = r#"Compute duration minutes using wake-date semantics in the provided timezone.

Given a target `wake_date`, `bed_time` and `wake_time`, the bed datetime may belong to the
previous calendar day when `bed_time > wake_time` (crossing midnight).

DST handling:
- Ambiguous local times (fall back) choose the earliest instant.
- Non-existent times (spring forward gap) advance minute-by-minute until valid.

# Example

```rust
# use std::error::Error;
# use chrono::{NaiveDate, NaiveTime};
# use chrono_tz::Asia::Tokyo;
# fn main() -> Result<(), Box<dyn Error>> {
// Cross-midnight: bed 23:00, wake 07:00 next day
let mins = sleep_api::time::compute_duration_min(
    NaiveDate::from_ymd_opt(2025, 6, 1).ok_or("invalid date")?,
    NaiveTime::from_hms_opt(23, 0, 0).ok_or("invalid time")?,
    NaiveTime::from_hms_opt(7, 0, 0).ok_or("invalid time")?,
    Tokyo,
)?;
assert_eq!(mins, 8 * 60);
# Ok(()) }
```

# Errors

Returns [`DomainError::InvalidInput`] if the computed duration is non-positive,
exceeds `i32::MAX`, or if the computed bed date would underflow.

[`DomainError::InvalidInput`]: crate::domain::DomainError::InvalidInput
"#]
pub fn compute_duration_min(
    wake_date: NaiveDate,
    bed_time: NaiveTime,
    wake_time: NaiveTime,
    tz: Tz,
) -> Result<i32, DomainError> {
    let bed_date = if bed_time > wake_time {
        wake_date
            .pred_opt()
            .ok_or_else(|| DomainError::InvalidInput("invalid date (underflow)".into()))?
    } else {
        wake_date
    };

    let bed_ndt = NaiveDateTime::new(bed_date, bed_time);
    let wake_ndt = NaiveDateTime::new(wake_date, wake_time);

    let bed_local = resolve_local(tz, bed_ndt);
    let wake_local = resolve_local(tz, wake_ndt);

    let bed_utc = bed_local.with_timezone(&Utc);
    let wake_utc = wake_local.with_timezone(&Utc);

    let mins = (wake_utc - bed_utc).num_minutes();
    if mins <= 0 {
        return Err(DomainError::InvalidInput(
            "Duration must be positive".into(),
        ));
    }
    if mins > i32::MAX as i64 {
        return Err(DomainError::InvalidInput("Duration too large".into()));
    }
    Ok(mins as i32)
}
