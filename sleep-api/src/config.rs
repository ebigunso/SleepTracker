#![doc = r#"Configuration utilities

Provides application configuration helpers such as the default timezone used by
time computations. See also: [`time::compute_duration_min`].

[`time::compute_duration_min`]: crate::time::compute_duration_min
"#]

use chrono_tz::Tz;
use std::str::FromStr;

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
pub fn admin_email() -> String {
    std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string())
}

/// Return the admin password hash from ADMIN_PASSWORD_HASH (argon2id string).
/// Returns empty string if unset, causing login to fail.
pub fn admin_password_hash() -> String {
    std::env::var("ADMIN_PASSWORD_HASH").unwrap_or_default()
}

/// Build a cookie Key from SESSION_SECRET if provided (base64), otherwise generate a random key.
/// A stable key is recommended for production to allow restarting without invalidating sessions.
pub fn session_key() -> axum_extra::extract::cookie::Key {
    use base64::{engine::general_purpose, Engine as _};
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
pub fn hsts_enabled() -> bool {
    match std::env::var("ENABLE_HSTS") {
        Ok(v) => v == "1" || v.eq_ignore_ascii_case("true"),
        Err(_) => false,
    }
}
