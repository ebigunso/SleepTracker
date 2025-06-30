use sleep_api::{db, app};
use sleep_api::models::{SleepInput, SleepSession};
use reqwest::Client;

#[tokio::test]
async fn test_sleep_flow() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    let pool = db::connect().await.unwrap();
    sqlx::migrate!("../migrations").run(&pool).await.unwrap();
    let app = app::router(pool.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(axum::serve(listener, app));

    let client = Client::new();
    let health_url = format!("http://{}/health", addr);
    let mut ready = false;
    for _ in 0..10 {
        if client.get(&health_url).send().await.is_ok() {
            ready = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    assert!(ready, "Server did not become ready in time");
    let input = SleepInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 17).unwrap(),
        bed_time: chrono::NaiveTime::from_hms_opt(23, 5, 0).unwrap(),
        wake_time: chrono::NaiveTime::from_hms_opt(6, 15, 0).unwrap(),
        latency_min: 10,
        awakenings: 1,
        quality: 4,
    };
    let res = client.post(&format!("http://{}/sleep", addr))
        .json(&input).send().await.unwrap();
    assert_eq!(res.status(), 201);
    let id: serde_json::Value = res.json().await.unwrap();
    let id = id["id"].as_i64().unwrap();

    let res = client.get(&format!("http://{}/sleep/{}", addr, input.date)).send().await.unwrap();
    assert_eq!(res.status(), 200);
    let session: SleepSession = res.json().await.unwrap();
    assert_eq!(session.id, id);
    server.abort();
}
