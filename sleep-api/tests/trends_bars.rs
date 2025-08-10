use reqwest::Client;
use sleep_api::{app, db};
use sleep_api::models::{Quality, SleepInput};

#[tokio::test]
async fn test_trends_sleep_bars_basic() {
    // In-memory DB
    unsafe { std::env::set_var("DATABASE_URL", "sqlite::memory:") };
    let pool = db::connect().await.unwrap();
    sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    // Start server
    let app = app::router(pool.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let _server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = Client::new();

    // Wait for health to be ready
    let health_url = format!("http://{}/health", addr);
    let mut ready = false;
    for _ in 0..10 {
        if client.get(&health_url).send().await.is_ok() {
            ready = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    assert!(ready, "server did not become ready");

    // Seed two sleep entries (wake-date semantics)
    let s1 = SleepInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 17).unwrap(),
        bed_time: chrono::NaiveTime::from_hms_opt(23, 5, 0).unwrap(),
        wake_time: chrono::NaiveTime::from_hms_opt(6, 15, 0).unwrap(),
        latency_min: 15,
        awakenings: 0,
        quality: Quality(4),
    };
    let s2 = SleepInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 18).unwrap(),
        bed_time: chrono::NaiveTime::from_hms_opt(0, 30, 0).unwrap(),
        wake_time: chrono::NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
        latency_min: 20,
        awakenings: 1,
        quality: Quality(3),
    };

    let res = client
        .post(&format!("http://{}/sleep", addr))
        .json(&s1)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);

    let res = client
        .post(&format!("http://{}/sleep", addr))
        .json(&s2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);

    // Call sleep-bars
    let bars_url = format!(
        "http://{}/api/trends/sleep-bars?from=2025-06-16&to=2025-06-19",
        addr
    );
    let res = client.get(&bars_url).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let bars_json: serde_json::Value = res.json().await.unwrap();
    assert!(bars_json.is_array());
    let arr = bars_json.as_array().unwrap();
    assert!(arr.len() >= 2);

    // Shape checks on first element
    let first = &arr[0];
    assert!(first.get("date").is_some(), "missing date");
    assert!(first.get("bed_time").is_some(), "missing bed_time");
    assert!(first.get("wake_time").is_some(), "missing wake_time");
}
