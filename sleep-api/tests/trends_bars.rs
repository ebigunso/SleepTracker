use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use reqwest::Client;
use sleep_api::models::{Quality, SleepInput};
use sleep_api::{app, db};

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
    let health_url = format!("http://{addr}/health");
    for _ in 0..20 {
        if client.get(&health_url).send().await.is_ok() {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    panic!("server did not become ready");
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
        .post(format!("http://{addr}/login.json"))
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .expect("login request failed");
    assert_eq!(res.status(), 200, "login failed: {}", res.status());
    let headers = res.headers().get_all(reqwest::header::SET_COOKIE);
    let csrf = parse_cookie(headers.iter(), "__Host-csrf=").expect("missing __Host-csrf cookie");
    let session =
        parse_cookie(headers.iter(), "__Host-session=").expect("missing __Host-session cookie");
    (csrf, session)
}

#[tokio::test]
async fn test_trends_sleep_bars_basic() {
    // In-memory DB
    unsafe { std::env::set_var("DATABASE_URL", "sqlite::memory:") };
    set_admin_env("admin@example.com", "password123");

    let pool = db::connect().await.unwrap();
    sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    // Start server
    let app = app::router(pool.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.2:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let _server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = Client::builder().cookie_store(true).build().unwrap();

    // Wait for health to be ready
    wait_ready(&client, &addr.to_string()).await;

    // Login and get CSRF + session
    let (csrf, session) = login_and_get_auth(
        &client,
        &addr.to_string(),
        "admin@example.com",
        "password123",
    )
    .await;

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
        .post(format!("http://{addr}/sleep"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .header("X-CSRF-Token", &csrf)
        .json(&s1)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);

    let res = client
        .post(format!("http://{addr}/sleep"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .header("X-CSRF-Token", &csrf)
        .json(&s2)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);

    // Call sleep-bars
    let bars_url = format!("http://{addr}/api/trends/sleep-bars?from=2025-06-16&to=2025-06-19");
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
