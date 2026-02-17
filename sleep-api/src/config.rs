#![doc = r#"Configuration utilities

Provides application configuration helpers such as the default timezone used by
time computations. See also: [`time::compute_duration_min`].

[`time::compute_duration_min`]: crate::time::compute_duration_min
"#]

use chrono_tz::Tz;
use std::str::FromStr;

fn env_flag(name: &str, default: bool) -> bool {
    match std::env::var(name) {
        Ok(v) => v == "1" || v.eq_ignore_ascii_case("true"),
        Err(_) => default,
    }
}

#[doc = r#"Return the application timezone derived from the `APP_TZ` environment variable.

If `APP_TZ` is not set or contains an unknown zone name, the function falls back to `Asia/Tokyo`.

This timezone is used by functions like [`time::compute_duration_min`] to interpret local
bed/wake times in a consistent, DST-aware manner.

# Example

```rust,no_run
# use std::error::Error;
# fn main() -> Result<(), Box<dyn Error>> {
# unsafe {
std::env::set_var("APP_TZ", "Asia/Tokyo");
let tz = sleep_api::config::app_tz();
assert_eq!(tz, chrono_tz::Asia::Tokyo);

// Unknown values also fall back to Asia/Tokyo.
std::env::set_var("APP_TZ", "Not/AZone");
let tz2 = sleep_api::config::app_tz();
assert_eq!(tz2, chrono_tz::Asia::Tokyo);
# }
# Ok(()) }
```

[`time::compute_duration_min`]: crate::time::compute_duration_min
"#]
pub fn app_tz() -> Tz {
    let name = std::env::var("APP_TZ").unwrap_or_else(|_| "Asia/Tokyo".to_string());
    Tz::from_str(&name).unwrap_or(chrono_tz::Asia::Tokyo)
}

/// Return the admin email from ADMIN_EMAIL (defaults to admin@example.com).
#[doc = r#"Return the admin email from the `ADMIN_EMAIL` environment variable.

Defaults to `admin@example.com` if unset.

# Example

```rust,no_run
# unsafe {
std::env::set_var("ADMIN_EMAIL", "owner@example.com");
assert_eq!(sleep_api::config::admin_email(), "owner@example.com");
# }
```"#]
pub fn admin_email() -> String {
    std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string())
}

/// Return the admin password hash from ADMIN_PASSWORD_HASH (argon2id string).
/// Returns empty string if unset, causing login to fail.
#[doc = r#"Return the admin password hash from `ADMIN_PASSWORD_HASH`.

Expected format is an Argon2id hash string (e.g., `$argon2id$...`). Returns empty string if unset,
which causes login verification to fail."#]
pub fn admin_password_hash() -> String {
    std::env::var("ADMIN_PASSWORD_HASH").unwrap_or_default()
}

/// Build a cookie Key from SESSION_SECRET if provided (base64), otherwise generate a random key.
/// A stable key is recommended for production to allow restarting without invalidating sessions.
#[doc = r#"Build a cookie [`Key`] from `SESSION_SECRET` (base64) or generate a random key.

A stable `SESSION_SECRET` is recommended for production to avoid invalidating sessions on restart.
If the environment variable is absent or invalid, a new random key is generated.

[`Key`]: axum_extra::extract::cookie::Key
"#]
pub fn session_key() -> axum_extra::extract::cookie::Key {
    use base64::{Engine as _, engine::general_purpose};
    if let Ok(val) = std::env::var("SESSION_SECRET") {
        match general_purpose::STANDARD.decode(val.as_bytes()) {
            Ok(bytes) => {
                return axum_extra::extract::cookie::Key::derive_from(&bytes);
            }
            Err(e) => {
                tracing::warn!(error = ?e, "Invalid base64 in SESSION_SECRET, generating random key");
            }
        }
    }
    axum_extra::extract::cookie::Key::generate()
}

/// Whether to enable the HSTS header. Controlled by ENABLE_HSTS=1/true.
#[doc = r#"Return whether to enable the HSTS header.

Reads `ENABLE_HSTS` and treats `1` or `true` (case-insensitive) as enabled.
Only enable when serving over HTTPS or behind TLS-terminating proxy."#]
pub fn hsts_enabled() -> bool {
    env_flag("ENABLE_HSTS", false)
}

/// Whether to mark cookies as Secure. Controlled by COOKIE_SECURE=1/true (default: true).
pub fn cookie_secure() -> bool {
    env_flag("COOKIE_SECURE", true) // default secure for safety
}

/// Whether `/api/trends/personalization` is exposed.
/// Controlled by `ENABLE_PERSONALIZATION_TRENDS=1/true` (default: false).
pub fn personalization_trends_enabled() -> bool {
    env_flag("ENABLE_PERSONALIZATION_TRENDS", false)
}

/// Whether `POST /api/personalization/friction-telemetry` is exposed.
/// Controlled by `ENABLE_PERSONALIZATION_FRICTION_TELEMETRY=1/true` (default: false).
pub fn personalization_friction_telemetry_enabled() -> bool {
    env_flag("ENABLE_PERSONALIZATION_FRICTION_TELEMETRY", false)
}

/// Whether `GET /api/personalization/friction-backlog` is exposed.
/// Controlled by `ENABLE_PERSONALIZATION_FRICTION_BACKLOG=1/true` (default: false).
pub fn personalization_friction_backlog_enabled() -> bool {
    env_flag("ENABLE_PERSONALIZATION_FRICTION_BACKLOG", false)
}

/// Session cookie name, varies in dev-mode to support HTTP.
/// - When cookie_secure() is true: "__Host-session"
/// - Otherwise: "session"
pub fn session_cookie_name() -> &'static str {
    if cookie_secure() {
        "__Host-session"
    } else {
        "session"
    }
}

/// CSRF cookie name, varies in dev-mode to support HTTP.
/// - When cookie_secure() is true: "__Host-csrf"
/// - Otherwise: "csrf"
pub fn csrf_cookie_name() -> &'static str {
    if cookie_secure() {
        "__Host-csrf"
    } else {
        "csrf"
    }
}

/// Optional session TTL (Max-Age) for the session cookie.
/// - Controlled by `SESSION_TTL_HOURS`
/// - Defaults to 12 hours when unset or invalid
/// - Set to "0" to disable Max-Age (session-only cookie)
pub fn session_ttl() -> Option<time::Duration> {
    match std::env::var("SESSION_TTL_HOURS") {
        Ok(v) => {
            let v = v.trim();
            if v.is_empty() {
                return Some(time::Duration::hours(12));
            }
            if v == "0" {
                return None;
            }
            match v.parse::<i64>() {
                Ok(h) if h > 0 => Some(time::Duration::hours(h)),
                Ok(_) => None,
                Err(e) => {
                    tracing::warn!(error=?e, value=%v, "Invalid SESSION_TTL_HOURS; using default 12h");
                    Some(time::Duration::hours(12))
                }
            }
        }
        Err(_) => Some(time::Duration::hours(12)),
    }
}
