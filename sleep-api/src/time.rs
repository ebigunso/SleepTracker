use crate::domain::DomainError;
use chrono::{
    DateTime, Duration as ChronoDuration, LocalResult, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Utc,
};
use chrono_tz::Tz;

/// Resolve a local naive datetime in a timezone to a concrete instant,
/// handling DST gaps/overlaps:
/// - Ambiguous: pick the earliest instant
/// - Non-existent (spring forward gap): advance in 1-minute steps until valid (max ~3 hours)
fn resolve_local(tz: Tz, ndt: NaiveDateTime) -> DateTime<Tz> {
    match tz.from_local_datetime(&ndt) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(earliest, _latest) => earliest,
        LocalResult::None => {
            // advance forward until it becomes valid
            let mut cur = ndt;
            for _ in 0..(3 * 60) {
                match tz.from_local_datetime(&cur) {
                    LocalResult::Single(dt) => return dt,
                    LocalResult::Ambiguous(earliest, _latest) => return earliest,
                    LocalResult::None => {
                        cur += ChronoDuration::minutes(1);
                    }
                }
            }
            // Fallback: interpret as UTC wall time then project to tz (best-effort)
            tz.from_utc_datetime(&ndt)
        }
    }
}

/// Compute duration minutes using wake-date semantics in the provided timezone:
/// bed_dt = (wake_date at bed_time) − (1 day if bed_time > wake_time else 0)
/// wake_dt = (wake_date at wake_time)
/// duration = wake_dt − bed_dt
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
