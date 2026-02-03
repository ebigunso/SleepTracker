use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use reqwest::Client;
use serial_test::serial;
use sleep_api::{app, db};
use tokio::time::{Duration, sleep};

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
        sleep(Duration::from_millis(100)).await;
    }
    panic!("server did not become ready");
}

fn parse_cookie<'a>(
    headers: impl Iterator<Item = &'a reqwest::header::HeaderValue>,
    name: &str,
) -> Option<String> {
    for hv in headers {
        if let Ok(s) = hv.to_str() {
            if s.starts_with(name) {
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

#[tokio::test]
#[serial]
async fn test_get_and_set_timezone() {
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        std::env::set_var("COOKIE_SECURE", "1");
        std::env::set_var("APP_TZ", "Asia/Tokyo");
    }
    set_admin_env("admin@example.com", "password123");

    let pool = db::connect().await.unwrap();
    sqlx::migrate::Migrator::new(std::path::Path::new("../migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    let app = app::router(pool.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let _server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    wait_ready(&client, &addr.to_string()).await;

    let login_body = serde_json::json!({
        "email": "admin@example.com",
        "password": "password123"
    });
    let login_res = client
        .post(format!("http://{addr}/api/login.json"))
        .json(&login_body)
        .send()
        .await
        .unwrap();
    assert_eq!(login_res.status(), 200);

    let csrf = parse_cookie(
        login_res
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter(),
        "__Host-csrf=",
    )
    .expect("missing __Host-csrf cookie in login response");
    let session = parse_cookie(
        login_res
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter(),
        "__Host-session=",
    )
    .expect("missing __Host-session cookie in login response");

    let res = client
        .get(format!("http://{addr}/api/settings/timezone"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["timezone"], "Asia/Tokyo");

    let res = client
        .post(format!("http://{addr}/api/settings/timezone"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .header("X-CSRF-Token", &csrf)
        .json(&serde_json::json!({ "timezone": "America/Los_Angeles" }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204);

    let res = client
        .get(format!("http://{addr}/api/settings/timezone"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["timezone"], "America/Los_Angeles");
}
