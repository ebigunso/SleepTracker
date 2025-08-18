#![doc = r#"Authentication extractors

Provides extractors to require a valid session:
- [`RequireSessionJson`] → returns `401` JSON (`{"error":"unauthorized"}`) on failure

These extractors read the encrypted `__Host-session` cookie via [`PrivateCookieJar`]. They require that the application state implements [`FromRef`] for [`Key`], which is provided by [`app::AppState`].

# Example

```rust,no_run
# use axum::{Json, response::IntoResponse};
# use sleep_api::middleware::auth_layer::RequireSessionJson;
# async fn api_handler(
#     RequireSessionJson { _user_id: _ }: RequireSessionJson,
#     Json(_): Json<serde_json::Value>,
# ) -> impl IntoResponse {
#     axum::http::StatusCode::NO_CONTENT
# }
```

See also:
- [`crate::auth`] for creating/clearing session cookies
- [`crate::security::csrf`] for CSRF protection guard
"#]

use axum::extract::{FromRef, FromRequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use serde_json::json;

use crate::auth::{UserId, current_user_from_cookie};

/// Extractor that requires an authenticated session for JSON APIs.
/// On failure, returns 401 with a JSON error payload.
pub struct RequireSessionJson {
    pub _user_id: UserId,
}

impl<S> FromRequestParts<S> for RequireSessionJson
where
    S: Send + Sync,
    Key: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| unauthorized())?;
        match current_user_from_cookie(&jar) {
            Some(uid) => Ok(Self { _user_id: uid }),
            None => Err(unauthorized()),
        }
    }
}

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        axum::Json(json!({"error":"unauthorized"})),
    )
        .into_response()
}
