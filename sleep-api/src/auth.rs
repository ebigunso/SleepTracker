use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use cookie as _;
use serde::Deserialize;

pub type UserId = String;

pub const SESSION_COOKIE: &str = "__Host-session";

/// Create a secure, HttpOnly session cookie storing the user id (encrypted via PrivateCookieJar).
pub fn create_session_cookie(mut jar: PrivateCookieJar, user_id: &str) -> PrivateCookieJar {
    let cookie = Cookie::build((SESSION_COOKIE, user_id.to_owned()))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    jar = jar.add(cookie);
    jar
}

/// Clear the session cookie.
pub fn clear_session_cookie(mut jar: PrivateCookieJar) -> PrivateCookieJar {
    // Removal needs to match name + path
    let cookie = Cookie::build((SESSION_COOKIE, String::new()))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    jar = jar.remove(cookie);
    jar
}

/// Return the current user id from the session cookie if present/valid.
pub fn current_user_from_cookie(jar: &PrivateCookieJar) -> Option<UserId> {
    jar.get(SESSION_COOKIE).map(|c| c.value().to_string())
}

/// Verify provided email + password against configured ADMIN_EMAIL + ADMIN_PASSWORD_HASH.
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
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}
