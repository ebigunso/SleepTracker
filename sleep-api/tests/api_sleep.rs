use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use reqwest::Client;
use sleep_api::models::{Quality, SleepInput, SleepSession};
use sleep_api::{app, db};
use sqlx::Row;

fn set_admin_env(email: &str, password: &str) {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    unsafe {
        std::env::set_var("ADMIN_EMAIL", email);
        std::env::set_var("ADMIN_PASSWORD_HASH", hash);
    }
}

async fn wait_ready(client: &Client, addr: &str) {
    let health_url = format!("http://{addr}/api/health");
    for _ in 0..20 {
        if client.get(&health_url).send().await.is_ok() {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    panic!("Server did not become ready in time");
}

fn parse_cookie<'a>(
    headers: impl Iterator<Item = &'a reqwest::header::HeaderValue>,
    name_with_eq: &str,
) -> Option<String> {
    for hv in headers {
        if let Ok(s) = hv.to_str() {
            if s.starts_with(name_with_eq) {
                if let Some(eq_idx) = s.find('=') {
                    let rest = &s[eq_idx + 1..];
                    let end = rest.find(';').unwrap_or(rest.len());
                    return Some(rest[..end].to_string());
                }
            }
        }
    }
    None
}

async fn login_and_get_auth(
    client: &Client,
    addr: &str,
    email: &str,
    password: &str,
) -> (String, String) {
    let res = client
        .post(format!("http://{addr}/api/login.json"))
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .expect("login request failed");
    assert_eq!(res.status(), 200, "login failed: {}", res.status());
    let headers = res.headers().get_all(reqwest::header::SET_COOKIE);
    // Accept both secure (__Host-*) and dev-mode (no prefix) cookie names
    let csrf = parse_cookie(headers.iter(), "__Host-csrf=")
        .or_else(|| parse_cookie(headers.iter(), "csrf="))
        .expect("missing CSRF cookie in login response");
    let session = parse_cookie(headers.iter(), "__Host-session=")
        .or_else(|| parse_cookie(headers.iter(), "session="))
        .expect("missing session cookie in login response");
    (csrf, session)
}

#[tokio::test]
async fn test_sleep_flow() {
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        std::env::set_var("COOKIE_SECURE", "0");
    };
    set_admin_env("admin@example.com", "password123");

    let pool = db::connect().await.unwrap();
    sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();
    let app = app::router(pool.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.2:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = Client::builder().cookie_store(true).build().unwrap();
    wait_ready(&client, &addr.to_string()).await;

    // Login and get CSRF token
    let (csrf, session_cookie) = login_and_get_auth(
        &client,
        &addr.to_string(),
        "admin@example.com",
        "password123",
    )
    .await;

    let input = SleepInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 17).unwrap(),
        bed_time: chrono::NaiveTime::from_hms_opt(22, 5, 0).unwrap(),
        wake_time: chrono::NaiveTime::from_hms_opt(23, 15, 0).unwrap(),
        latency_min: 10,
        awakenings: 1,
        quality: Quality(4),
    };
    let res = client
        .post(format!("http://{addr}/api/sleep"))
        .header("Cookie", format!("session={session_cookie}; csrf={csrf}"))
        .header("X-CSRF-Token", &csrf)
        .json(&input)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let id: serde_json::Value = res.json().await.unwrap();
    let id = id["id"].as_i64().unwrap();

    let res = client
        .get(format!("http://{addr}/api/sleep/date/{}", input.date))
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
        .put(format!("http://{addr}/api/sleep/{id}"))
        .header("Cookie", format!("session={session_cookie}; csrf={csrf}"))
        .header("X-CSRF-Token", &csrf)
        .json(&updated)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204);

    let res = client
        .get(format!("http://{addr}/api/sleep/date/{}", updated.date))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    session = res.json().await.unwrap();
    assert_eq!(session.quality, 5);
    assert_eq!(session.latency_min, updated.latency_min);

    let res = client
        .delete(format!("http://{addr}/api/sleep/{id}"))
        .header("Cookie", format!("session={session_cookie}; csrf={csrf}"))
        .header("X-CSRF-Token", &csrf)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204);

    // Idempotency: deleting the same id again should still return 204
    let res = client
        .delete(format!("http://{addr}/api/sleep/{id}"))
        .header("Cookie", format!("session={session_cookie}; csrf={csrf}"))
        .header("X-CSRF-Token", &csrf)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204, "idempotent delete should be 204");

    let res = client
        .get(format!("http://{addr}/api/sleep/date/{}", updated.date))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);

    server.abort();
}

#[tokio::test]
async fn test_exercise_and_note() {
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        std::env::set_var("COOKIE_SECURE", "0");
    };
    set_admin_env("admin@example.com", "password123");

    let pool = db::connect().await.unwrap();
    sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();
    let app = app::router(pool.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.2:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = Client::builder().cookie_store(true).build().unwrap();
    wait_ready(&client, &addr.to_string()).await;

    // Login and get CSRF token
    let (csrf, session_cookie) = login_and_get_auth(
        &client,
        &addr.to_string(),
        "admin@example.com",
        "password123",
    )
    .await;

    let exercise = sleep_api::models::ExerciseInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 17).unwrap(),
        intensity: sleep_api::models::intensity::Intensity::Light,
        start_time: Some(chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap()),
        duration_min: Some(30),
    };
    let res = client
        .post(format!("http://{addr}/api/exercise"))
        .header("Cookie", format!("session={session_cookie}; csrf={csrf}"))
        .header("X-CSRF-Token", &csrf)
        .json(&exercise)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let val: serde_json::Value = res.json().await.unwrap();
    let ex_id = val["id"].as_i64().unwrap();

    let row = sqlx::query("SELECT intensity, duration_min FROM exercise_events WHERE id = ?")
        .bind(ex_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let intensity: String = row.get(0);
    let duration: Option<i32> = row.get(1);
    assert_eq!(intensity, "light");
    assert_eq!(duration, Some(30));

    let note = sleep_api::models::NoteInput {
        date: exercise.date,
        body: Some("Great workout".to_string()),
    };
    let res = client
        .post(format!("http://{addr}/api/note"))
        .header("Cookie", format!("session={session_cookie}; csrf={csrf}"))
        .header("X-CSRF-Token", &csrf)
        .json(&note)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let val: serde_json::Value = res.json().await.unwrap();
    let note_id = val["id"].as_i64().unwrap();

    let row = sqlx::query("SELECT body FROM notes WHERE id = ?")
        .bind(note_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let body: Option<String> = row.get(0);
    assert_eq!(body, Some("Great workout".to_string()));

    server.abort();
}
