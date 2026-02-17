use chrono::{NaiveDate, NaiveDateTime};
use sleep_api::models::FrictionTelemetryInput;
use sleep_api::{db, repository};

fn dt(y: i32, m: u32, d: u32) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(y, m, d)
        .expect("valid date")
        .and_hms_opt(0, 0, 0)
        .expect("valid datetime")
}

fn approx_eq(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "left={left}, right={right}");
}

#[tokio::test]
async fn test_insert_and_aggregate_friction_window() {
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
    }
    let pool = db::connect().await.expect("db connect");
    sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
        .await
        .expect("migrator")
        .run(&pool)
        .await
        .expect("migrations run");

    let inputs = [
        FrictionTelemetryInput {
            form_time_ms: 1000,
            error_kind: None,
            retry_count: 0,
            immediate_edit: false,
            follow_up_failure: false,
        },
        FrictionTelemetryInput {
            form_time_ms: 2000,
            error_kind: Some("validation".to_string()),
            retry_count: 1,
            immediate_edit: true,
            follow_up_failure: false,
        },
        FrictionTelemetryInput {
            form_time_ms: 6000,
            error_kind: Some("validation ".to_string()),
            retry_count: 2,
            immediate_edit: true,
            follow_up_failure: true,
        },
    ];

    for input in &inputs {
        let id = repository::insert_friction_telemetry(&pool, input)
            .await
            .expect("insert friction telemetry");
        assert!(id > 0);
    }

    let from = dt(1970, 1, 1);
    let to = dt(2100, 1, 1);

    let events = repository::list_friction_telemetry_window(&pool, from, to)
        .await
        .expect("list friction telemetry window");
    assert_eq!(events.len(), 3);
    assert!(events[0].id < events[1].id && events[1].id < events[2].id);
    assert_eq!(events[0].form_time_ms, 1000);
    assert_eq!(events[1].form_time_ms, 2000);
    assert_eq!(events[2].form_time_ms, 6000);

    let agg = repository::aggregate_friction_window(&pool, from, to)
        .await
        .expect("aggregate friction window");
    assert_eq!(agg.submit_count, 3);
    approx_eq(agg.median_form_time_ms, 2000.0);
    approx_eq(agg.avg_form_time_ms, 3000.0);
    assert_eq!(agg.error_count, 2);
    assert_eq!(agg.retries_total, 3);
    approx_eq(agg.retries_avg, 1.0);
    assert_eq!(agg.immediate_edit_count, 2);
    assert_eq!(agg.follow_up_failure_count, 1);
    approx_eq(agg.error_rate, 2.0 / 3.0);
    approx_eq(agg.immediate_edit_rate, 2.0 / 3.0);
    approx_eq(agg.follow_up_failure_rate, 1.0 / 3.0);
}

#[tokio::test]
async fn test_aggregate_friction_window_empty_range_returns_zeros() {
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
    }
    let pool = db::connect().await.expect("db connect");
    sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
        .await
        .expect("migrator")
        .run(&pool)
        .await
        .expect("migrations run");

    let agg = repository::aggregate_friction_window(&pool, dt(1970, 1, 1), dt(1970, 1, 2))
        .await
        .expect("aggregate friction window");

    assert_eq!(agg.submit_count, 0);
    approx_eq(agg.median_form_time_ms, 0.0);
    approx_eq(agg.avg_form_time_ms, 0.0);
    assert_eq!(agg.error_count, 0);
    assert_eq!(agg.retries_total, 0);
    approx_eq(agg.retries_avg, 0.0);
    assert_eq!(agg.immediate_edit_count, 0);
    assert_eq!(agg.follow_up_failure_count, 0);
    approx_eq(agg.error_rate, 0.0);
    approx_eq(agg.immediate_edit_rate, 0.0);
    approx_eq(agg.follow_up_failure_rate, 0.0);
}
