use argon2::password_hash::rand_core::OsRng;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use reqwest::Client;
use serial_test::serial;
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
#[serial]
async fn test_auth_and_csrf_flow() {
    // Env & DB
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        std::env::set_var("COOKIE_SECURE", "1");
    };
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

    // HEAD /health should be OK
    let res = client
        .head(format!("http://{addr}/api/health"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // Unauthenticated session probe should be false
    let res = client
        .get(format!("http://{addr}/api/session"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let unauth: serde_json::Value = res.json().await.unwrap();
    assert_eq!(
        unauth["authenticated"], false,
        "expected unauthenticated at start"
    );

    // Login with JSON
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

    // After login, /api/session should report authenticated: true
    let res = client
        .get(format!("http://{addr}/api/session"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let authed: serde_json::Value = res.json().await.unwrap();
    assert_eq!(authed["authenticated"], true);

    // Extract CSRF and session cookies from Set-Cookie headers
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
        .post(format!("http://{addr}/api/sleep"))
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
        .post(format!("http://{addr}/api/sleep"))
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
        .post(format!("http://{addr}/api/logout"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .header("X-CSRF-Token", &csrf)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 204);

    // After logout, /api/session should report false
    let res = client
        .get(format!("http://{addr}/api/session"))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let v3: serde_json::Value = res.json().await.unwrap();
    assert_eq!(v3["authenticated"], false);

    let res = client
        .delete(format!("http://{addr}/api/sleep/{id}"))
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

#[tokio::test]
#[serial]
async fn test_csrf_percent_encoded_header() {
    // Env & DB
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        std::env::set_var("COOKIE_SECURE", "1");
    };
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

    // Login to get CSRF + session
    let login_body = serde_json::json!({
        "email": "admin@example.com",
        "password": "password123"
    });
    let res = client
        .post(format!("http://{addr}/api/login.json"))
        .json(&login_body)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

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

    // Percent-encode characters commonly seen in URL-safe base64
    let encoded = csrf.replace('-', "%2D").replace('_', "%5F");

    // Mutating API with percent-encoded header should succeed (percent-decoded server-side)
    let sample = SleepInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 18).unwrap(),
        bed_time: chrono::NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
        wake_time: chrono::NaiveTime::from_hms_opt(6, 30, 0).unwrap(),
        latency_min: 12,
        awakenings: 0,
        quality: Quality(4),
    };
    let res = client
        .post(format!("http://{addr}/api/sleep"))
        .header(
            "Cookie",
            format!("__Host-session={session}; __Host-csrf={csrf}"),
        )
        .header("X-CSRF-Token", &encoded)
        .json(&sample)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}

#[tokio::test]
#[serial]
async fn test_dev_cookie_names_and_flags() {
    // Force dev-mode cookies (no Secure; names without __Host-)
    unsafe {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
        std::env::set_var("COOKIE_SECURE", "0");
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
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    wait_ready(&client, &addr.to_string()).await;

    let res = client
        .post(format!("http://{addr}/api/login.json"))
        .json(&serde_json::json!({ "email":"admin@example.com", "password":"password123" }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // Collect Set-Cookie headers as strings
    let set_cookies: Vec<String> = res
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .filter_map(|h| h.to_str().ok().map(|s| s.to_string()))
        .collect();

    // Expect "session=" and "csrf=" cookies (not __Host- prefix)
    let has_session = set_cookies.iter().any(|s| s.starts_with("session="));
    let has_csrf = set_cookies.iter().any(|s| s.starts_with("csrf="));
    assert!(has_session, "expected session= cookie in dev mode");
    assert!(has_csrf, "expected csrf= cookie in dev mode");

    // Ensure cookies do NOT include Secure attribute in dev mode
    for sc in &set_cookies {
        if sc.starts_with("session=") || sc.starts_with("csrf=") {
            assert!(
                !sc.contains("Secure"),
                "dev cookies should not include Secure: {sc}"
            );
        }
    }

    // Sanity: use these cookies to perform a mutating call
    let csrf_val = parse_cookie(
        res.headers().get_all(reqwest::header::SET_COOKIE).iter(),
        "csrf=",
    )
    .expect("missing csrf cookie");
    let session_val = parse_cookie(
        res.headers().get_all(reqwest::header::SET_COOKIE).iter(),
        "session=",
    )
    .expect("missing session cookie");

    let sample = SleepInput {
        date: chrono::NaiveDate::from_ymd_opt(2025, 6, 19).unwrap(),
        bed_time: chrono::NaiveTime::from_hms_opt(22, 30, 0).unwrap(),
        wake_time: chrono::NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        latency_min: 10,
        awakenings: 0,
        quality: Quality(5),
    };
    let res = client
        .post(format!("http://{addr}/api/sleep"))
        .header("Cookie", format!("session={session_val}; csrf={csrf_val}"))
        .header("X-CSRF-Token", &csrf_val)
        .json(&sample)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
}
