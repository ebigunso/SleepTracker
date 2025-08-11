use axum::extract::FromRequestParts;
use axum::http::{header::HeaderName, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde_json::json;

pub const CSRF_COOKIE: &str = "__Host-csrf";

/// Issue a CSRF cookie with a random 32-byte base64 value.
/// - Secure
/// - SameSite=Lax
/// - Path=/
/// - Not HttpOnly (so a future UI may read and echo it via X-CSRF-Token)
pub fn issue_csrf_cookie() -> Cookie<'static> {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let token = base64::engine::general_purpose::STANDARD.encode(bytes);

    Cookie::build((CSRF_COOKIE, token))
        .path("/")
        .secure(true)
        .http_only(false)
        .same_site(SameSite::Lax)
        .build()
}

/// Guard extractor that enforces double-submit CSRF for mutating methods (POST/PUT/DELETE).
/// - Requires a cookie "__Host-csrf"
/// - Requires header "X-CSRF-Token" matching the cookie value
/// - If "Sec-Fetch-Site" header is present, it must be "same-origin" or "same-site"
pub struct CsrfGuard;

impl<S> FromRequestParts<S> for CsrfGuard
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Only enforce on mutating methods
        let method = parts.method.clone();
        let is_mutating = matches!(method, Method::POST | Method::PUT | Method::DELETE);
        if !is_mutating {
            return Ok(Self);
        }

        // Basic same-site heuristic via Sec-Fetch-Site if provided
        if let Some(h) = parts.headers.get("sec-fetch-site") {
            if let Ok(v) = h.to_str() {
                let v = v.to_ascii_lowercase();
                if v != "same-origin" && v != "same-site" {
                    return Err(forbidden("csrf: cross-site request rejected"));
                }
            }
        }

        // Read CSRF cookie
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .unwrap_or_else(|_| CookieJar::new());
        let cookie_val = match jar.get(CSRF_COOKIE) {
            Some(c) => c.value().to_string(),
            None => return Err(forbidden("csrf: missing cookie")),
        };

        // Compare against header X-CSRF-Token
        static X_CSRF_TOKEN: HeaderName = HeaderName::from_static("x-csrf-token");
        let hdr = parts.headers.get(&X_CSRF_TOKEN)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let Some(token) = hdr else {
            return Err(forbidden("csrf: missing header token"));
        };

        if token != cookie_val {
            return Err(forbidden("csrf: token mismatch"));
        }

        Ok(Self)
    }
}

fn forbidden(detail: &str) -> Response {
    (StatusCode::FORBIDDEN, axum::Json(json!({"error":"forbidden","detail": detail}))).into_response()
}
