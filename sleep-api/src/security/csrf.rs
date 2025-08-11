#![doc = r#"CSRF protection (double-submit)

Implements double-submit cookie protection for mutating requests:

- Cookie `__Host-csrf` (Secure, SameSite=Lax, Path=/, not HttpOnly), value: URL-safe base64 token
- Header `X-CSRF-Token` must match the cookie value (header is percent-decoded before comparison)
- For mutating requests (POST, PUT, DELETE), [`CsrfGuard`] enforces:
  - Same-site heuristic using `Sec-Fetch-Site` if present (`same-origin` or `same-site`)
  - Exact match of header token to cookie value (after percent-decoding)

# Example

```rust,no_run
# use axum::{Json, response::IntoResponse};
# use sleep_api::middleware::auth_layer::RequireSessionJson;
# use sleep_api::security::csrf::CsrfGuard;
async fn post_thing(
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(_): Json<serde_json::Value>,
) -> impl IntoResponse {
    axum::http::StatusCode::NO_CONTENT
}
```

See also:
- [`issue_csrf_cookie`] for issuing the CSRF cookie on login
"#]

use axum::extract::FromRequestParts;
use axum::http::{Method, StatusCode, header::HeaderName};
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use base64::Engine;
use rand::RngCore;
use serde_json::json;

#[doc = r#"CSRF cookie name.

- Name: `__Host-csrf`
- Attributes: Secure, SameSite=Lax, Path=/
- Not HttpOnly (so a UI can echo the value into `X-CSRF-Token` when needed)"#]
pub const CSRF_COOKIE: &str = "__Host-csrf";

/// Issue a CSRF cookie with a random 32-byte base64 value.
/// - Secure
/// - SameSite=Lax
/// - Path=/
/// - Not HttpOnly (so a future UI may read and echo it via X-CSRF-Token)
#[doc = r#"Issue a CSRF cookie with a random 32-byte URL-safe base64 value.

Cookie attributes:
- Secure
- SameSite=Lax
- Path=/
- Not HttpOnly

Returns a cookie ready to be added to a [`CookieJar`]."#]
pub fn issue_csrf_cookie() -> Cookie<'static> {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    let token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

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
#[doc = r#"Extractor that enforces double-submit CSRF for mutating methods (POST/PUT/DELETE).

Enforcement:
- If `Sec-Fetch-Site` header is present, it must be `same-origin` or `same-site`
- Reads `__Host-csrf` cookie and compares it to `X-CSRF-Token` header (header is percent-decoded before comparison)
- On failure, returns `403` with JSON payload: `{"error":"forbidden","detail":"csrf: ..."}`
"#]
pub struct CsrfGuard;

impl<S> FromRequestParts<S> for CsrfGuard
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
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
        let hdr = parts
            .headers
            .get(&X_CSRF_TOKEN)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let Some(token_raw) = hdr else {
            return Err(forbidden("csrf: missing header token"));
        };

        // Some intermediaries/clients percent-encode cookie values like "/" as "%2F".
        // Decode percent-encodings in the header token before comparing.
        let token = if token_raw.contains('%') {
            match percent_decode(&token_raw) {
                Some(s) => s,
                None => token_raw.clone(),
            }
        } else {
            token_raw.clone()
        };

        // Debug lengths to help diagnose mismatches during tests
        eprintln!(
            "csrf debug: cookie_len={}, token_len={}",
            cookie_val.len(),
            token.len()
        );
        if token != cookie_val {
            eprintln!(
                "csrf debug: cookie_prefix={:?}, token_prefix={:?}",
                &cookie_val.chars().take(8).collect::<String>(),
                &token.chars().take(8).collect::<String>()
            );
            return Err(forbidden("csrf: token mismatch"));
        }

        Ok(Self)
    }
}

fn percent_decode(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if i + 2 >= bytes.len() {
                return None;
            }
            let h1 = (bytes[i + 1] as char).to_digit(16)?;
            let h2 = (bytes[i + 2] as char).to_digit(16)?;
            let val = ((h1 << 4) + h2) as u8;
            out.push(val);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

fn forbidden(detail: &str) -> Response {
    (
        StatusCode::FORBIDDEN,
        axum::Json(json!({"error":"forbidden","detail": detail})),
    )
        .into_response()
}
