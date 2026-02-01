use chrono::{NaiveDate, NaiveTime};
use sleep_api::{domain::DomainError, models::Intensity};

#[test]
fn exercise_duration_negative_is_invalid() {
    let input = sleep_api::models::ExerciseInput {
        date: NaiveDate::from_ymd_opt(2025, 6, 17).expect("valid date"),
        intensity: Intensity::Light,
        start_time: Some(NaiveTime::from_hms_opt(9, 0, 0).expect("valid time")),
        duration_min: Some(-5),
    };

    let err = input.validate().expect_err("negative duration should be rejected");
    assert!(matches!(err, DomainError::InvalidInput(_)));
}

#[test]
fn exercise_duration_excessive_is_invalid() {
    let input = sleep_api::models::ExerciseInput {
        date: NaiveDate::from_ymd_opt(2025, 6, 17).expect("valid date"),
        intensity: Intensity::Hard,
        start_time: Some(NaiveTime::from_hms_opt(18, 30, 0).expect("valid time")),
        duration_min: Some(24 * 60 + 1),
    };

    let err = input
        .validate()
        .expect_err("excessive duration should be rejected");
    assert!(matches!(err, DomainError::InvalidInput(_)));
}
