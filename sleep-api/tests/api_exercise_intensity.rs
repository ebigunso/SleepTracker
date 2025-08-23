use argon2::password_hash::rand_core::OsRng;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use reqwest::Client;
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

#[derive(serde::Deserialize, Debug)]
struct DateIntensity {
    date: chrono::NaiveDate,
    intensity: String, // "none" | "light" | "hard"
}

#[tokio::test]
async fn test_exercise_intensity_range_endpoint() {
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

    let (csrf, session_cookie) =
        login_and_get_auth(&client, &addr.to_string(), "admin@example.com", "password123").await;

    // Seed exercise events:
    // 2025-06-10: light
    // 2025-06-11: none
    // 2025-06-12: light then hard (final should be "hard")
    let seeds = vec![
        ("2025-06-10", "light"),
        ("2025-06-11", "none"),
        ("2025-06-12", "light"),
        ("2025-06-12", "hard"),
    ];
    for (date, intensity) in seeds {
        let res = client
            .post(format!("http://{addr}/api/exercise"))
            .header("Cookie", format!("session={session_cookie}; csrf={csrf}"))
            .header("X-CSRF-Token", &csrf)
            .json(&serde_json::json!({
                "date": date,
                "intensity": intensity
            }))
            .send()
            .await
            .unwrap();
        let status = res.status();
        if status != 201 {
            let body = res.text().await.unwrap_or_else(|_| "<no body>".into());
            panic!("seed exercise failed: {} body: {}", status, body);
        }
    }

    // Query intensities in [2025-06-10, 2025-06-12]
    let res = client
        .get(format!(
            "http://{addr}/api/exercise/intensity?from=2025-06-10&to=2025-06-12"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200, "intensity status {}", res.status());
    let items: Vec<DateIntensity> = res.json().await.unwrap();
    assert_eq!(items.len(), 3, "expected 3 days");
    assert_eq!(items[0].date, chrono::NaiveDate::from_ymd_opt(2025, 6, 10).unwrap());
    assert_eq!(items[0].intensity, "light");
    assert_eq!(items[1].date, chrono::NaiveDate::from_ymd_opt(2025, 6, 11).unwrap());
    assert_eq!(items[1].intensity, "none");
    assert_eq!(items[2].date, chrono::NaiveDate::from_ymd_opt(2025, 6, 12).unwrap());
    assert_eq!(items[2].intensity, "hard");

    // Invalid range: from > to => 400
    let res = client
        .get(format!(
            "http://{addr}/api/exercise/intensity?from=2025-06-13&to=2025-06-12"
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 400);

    server.abort();
}
