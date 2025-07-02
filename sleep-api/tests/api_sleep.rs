use reqwest::Client;
use sleep_api::models::{SleepInput, SleepSession, Quality};
use sleep_api::{app, db};

#[tokio::test]
async fn test_sleep_flow() {
    unsafe { std::env::set_var("DATABASE_URL", "sqlite::memory:") };
    let pool = db::connect().await.unwrap();
    sqlx::migrate!("../migrations").run(&pool).await.unwrap();
    let app = app::router(pool.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

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
        quality: Quality(4),
    };
    let res = client
        .post(&format!("http://{}/sleep", addr))
        .json(&input)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let id: serde_json::Value = res.json().await.unwrap();
    let id = id["id"].as_i64().unwrap();

    let res = client
        .get(&format!("http://{}/sleep/date/{}", addr, input.date))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let mut session: SleepSession = res.json().await.unwrap();
    assert_eq!(session.id, id);
    assert_eq!(session.wake_time, input.wake_time);
    assert_eq!(session.latency_min, input.latency_min);
    assert_eq!(session.quality, input.quality.value() as i32);

    let updated = SleepInput {
        quality: Quality(5),
        ..input.clone()
    };
    let res = client
        .put(&format!("http://{}/sleep/{}", addr, id))
        .json(&updated)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204);

    let res = client
        .get(&format!("http://{}/sleep/date/{}", addr, updated.date))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    session = res.json().await.unwrap();
    assert_eq!(session.quality, 5);
    assert_eq!(session.latency_min, updated.latency_min);

    let res = client
        .delete(&format!("http://{}/sleep/{}", addr, id))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204);

    let res = client
        .get(&format!("http://{}/sleep/date/{}", addr, updated.date))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);

    server.abort();
}
