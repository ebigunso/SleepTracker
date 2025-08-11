#![doc = r#"Authentication utilities

Provides helpers for issuing and validating a session cookie (`__Host-session`) and verifying admin credentials.

Cookie:
- Name: `__Host-session`
- Attributes: Secure, HttpOnly, SameSite=Lax, Path=/
- Signed and encrypted via [`PrivateCookieJar`] using a key derived from `SESSION_SECRET`.

Admin login:
- `ADMIN_EMAIL`
- `ADMIN_PASSWORD_HASH` (`$argon2id$...`)

See also:
- [`security::csrf`] for CSRF token management and enforcement
- [`middleware::auth_layer`] for session-required extractors
"#]

use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use cookie as _;
use serde::Deserialize;

#[doc = r#"Single-user identifier.

This project supports a single admin user; `UserId` is typically `"admin"` or the configured `ADMIN_EMAIL`."#]
pub type UserId = String;


/// Create a secure, HttpOnly session cookie storing the user id (encrypted via PrivateCookieJar).
#[doc = r#"Create a secure, HttpOnly session cookie storing the user id.

The cookie is signed and encrypted via [`PrivateCookieJar`]. Returns the updated jar.

# Example

```rust,no_run
# use axum_extra::extract::cookie::PrivateCookieJar;
# fn demo(mut jar: PrivateCookieJar) -> PrivateCookieJar {
sleep_api::auth::create_session_cookie(jar, "admin")
# }
```"#]
pub fn create_session_cookie(mut jar: PrivateCookieJar, user_id: &str) -> PrivateCookieJar {
    let mut builder = Cookie::build((crate::config::session_cookie_name(), user_id.to_owned()))
        .path("/")
        .secure(crate::config::cookie_secure())
        .http_only(true)
        .same_site(SameSite::Lax);
    if let Some(ttl) = crate::config::session_ttl() {
        builder = builder.max_age(ttl);
    }
    let cookie = builder.build();
    jar = jar.add(cookie);
    jar
}

/// Clear the session cookie.
#[doc = r#"Clear the session cookie.

Sets a removal cookie (matching name + path) and returns the updated jar."#]
pub fn clear_session_cookie(mut jar: PrivateCookieJar) -> PrivateCookieJar {
    // Removal needs to match name + path
    let cookie = Cookie::build((crate::config::session_cookie_name(), String::new()))
        .path("/")
        .secure(crate::config::cookie_secure())
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    jar = jar.remove(cookie);
    jar
}

/// Return the current user id from the session cookie if present/valid.
#[doc = r#"Return the current user id from the encrypted session cookie, if present."#]
pub fn current_user_from_cookie(jar: &PrivateCookieJar) -> Option<UserId> {
    jar.get(crate::config::session_cookie_name())
        .map(|c| c.value().to_string())
}

/// Verify provided email + password against configured ADMIN_EMAIL + ADMIN_PASSWORD_HASH.
#[doc = r#"Verify provided `email` and `password` against configured admin credentials.

Reads:
- `ADMIN_EMAIL`
- `ADMIN_PASSWORD_HASH` (`$argon2id$...`)

Returns `true` on a valid match; otherwise `false`."#]
pub fn verify_login(email: &str, password: &str) -> bool {
    use argon2::{
        Argon2,
        password_hash::{PasswordHash, PasswordVerifier},
    };

    let admin_email = crate::config::admin_email();
    if email != admin_email {
        return false;
    }
    let hash = crate::config::admin_password_hash();
    if hash.is_empty() {
        // Lock out if not configured
        return false;
    }
    let parsed = match PasswordHash::new(&hash) {
        Ok(p) => p,
        Err(e) => {
            tracing::warn!(error=?e, "invalid ADMIN_PASSWORD_HASH value");
            return false;
        }
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

#[derive(Debug, Deserialize)]
#[doc = r#"Login request payload (JSON or form)."#]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
