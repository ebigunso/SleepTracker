use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use reqwest::Client;
use sleep_api::models::{Quality, SleepInput};
use sleep_api::{app, db};
use tokio::time::{Duration, sleep};

fn set_admin_env(email: &str, password: &str) {
    // Generate an argon2id hash for the given password and set envs
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
            // Set-Cookie can look like: "__Host-csrf=BASE64; Path=/; Secure; SameSite=Lax"
            if s.starts_with(name) {
                // extract value between name= and next ; or end
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
async fn test_auth_and_csrf_flow() {
    // Env & DB
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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let _server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Client with cookie store
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    wait_ready(&client, &addr.to_string()).await;

    // Unauthed UI route should redirect to /login
    let res = client
        .get(format!("http://{addr}/trends"))
        .send()
        .await
        .unwrap();
    assert!(
        res.status().is_redirection(),
        "expected redirect status, got {}",
        res.status()
    );
    let loc = res
        .headers()
        .get(reqwest::header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(
        loc.ends_with("/login"),
        "expected redirect to /login, got {loc}"
    );

    // Login with JSON
    let login_body = serde_json::json!({
        "email": "admin@example.com",
        "password": "password123"
    });
    let res = client
        .post(format!("http://{addr}/login"))
        .json(&login_body)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // Extract CSRF and session cookies from Set-Cookie headers
    let csrf = parse_cookie(
        res.headers().get_all(reqwest::header::SET_COOKIE).iter(),
        "__Host-csrf=",
    )
    .expect("missing __Host-csrf cookie in login response");
    let session = parse_cookie(
        res.headers().get_all(reqwest::header::SET_COOKIE).iter(),
        "__Host-session=",
    )
    .expect("missing __Host-session cookie in login response");

    // Mutating API without CSRF header should be 403
    let sample = SleepInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 17).unwrap(),
        bed_time: chrono::NaiveTime::from_hms_opt(23, 5, 0).unwrap(),
        wake_time: chrono::NaiveTime::from_hms_opt(6, 30, 0).unwrap(),
        latency_min: 15,
        awakenings: 0,
        quality: Quality(4),
    };
    let res = client
        .post(format!("http://{addr}/sleep"))
        .json(&sample)
        .send()
        .await
        .unwrap();
    assert_eq!(
        res.status(),
        403,
        "posting without X-CSRF-Token must be 403"
    );

    // Mutating API with CSRF header should succeed
    let res = client
        .post(format!("http://{addr}/sleep"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .header("X-CSRF-Token", &csrf)
        .json(&sample)
        .send()
        .await
        .unwrap();
    assert_eq!(
        res.status(),
        201,
        "posting with matching CSRF should succeed"
    );
    let v: serde_json::Value = res.json().await.unwrap();
    let id = v["id"].as_i64().unwrap();

    // Logout clears session; subsequent mutating should be 401
    let res = client
        .post(format!("http://{addr}/logout"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204);

    let res = client
        .delete(format!("http://{addr}/sleep/{id}"))
        .header("X-CSRF-Token", &csrf)
        .send()
        .await
        .unwrap();
    assert_eq!(
        res.status(),
        401,
        "after logout, mutating should be unauthorized"
    );
}
