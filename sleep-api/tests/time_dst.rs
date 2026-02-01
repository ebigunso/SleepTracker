use chrono::{NaiveDate, NaiveTime};
use chrono_tz::America::New_York;

#[test]
fn fall_back_same_local_times_yield_positive_duration() {
    let wake_date = NaiveDate::from_ymd_opt(2025, 11, 2).expect("valid date");
    let bed_time = NaiveTime::from_hms_opt(1, 30, 0).expect("valid time");
    let wake_time = NaiveTime::from_hms_opt(1, 30, 0).expect("valid time");

    let mins = sleep_api::time::compute_duration_min(wake_date, bed_time, wake_time, New_York)
        .expect("duration should be positive during fall back overlap");

    assert_eq!(mins, 60);
}
