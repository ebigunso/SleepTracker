#![doc = r#"HTTP routing

Defines the Axum [`Router`] that exposes the SleepTracker API. This module wires
all HTTP routes (health check, sleep CRUD, exercise, notes, and trends).

See the OpenAPI specification for request/response details:
- <https://github.com/ebigunso/SleepTracker/blob/main/openapi.yaml>

For an end-to-end server setup example, see [`router`].

[`Router`]: axum::Router
"#]

use crate::auth::{self, LoginPayload, current_user_from_cookie};
use crate::middleware::auth_layer::RequireSessionJson;
use crate::security::csrf::{CsrfGuard, issue_csrf_cookie};
use crate::{
    db::Db,
    error::ApiError,
    handlers,
    models::{ExerciseInput, NoteInput, SleepInput},
    trends,
};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::{
    Json, Router,
    extract::{Form, Path, State},
    routing::{get, post, put, head},
};
use axum_extra::extract::cookie::{Cookie, Key, PrivateCookieJar, SameSite};
use serde_json::json;

#[doc = r#"Build the application [`Router`].

Routes:
- `GET /health`
- `POST /sleep`
- `GET /sleep/date/{date}`
- `PUT /sleep/{id}`
- `DELETE /sleep/{id}`
- `POST /exercise`
- `POST /note`
- `GET /api/trends/sleep-bars`
- `GET /api/trends/summary`
- `GET /trends`

# Example

```rust,no_run
# use std::error::Error;
# async fn demo() -> Result<(), Box<dyn Error>> {
# // Acquire a database connection pool (for demonstration only).
let db = sleep_api::db::connect().await?;
let app = sleep_api::app::router(db);

// Now serve `app` with Axum/Hyper (listener creation elided).
// hyper::Server::bind(&addr).serve(app.into_make_service()).await?;
# Ok(())
# }
```

[`Router`]: axum::Router
"#]
#[derive(Clone)]
#[doc = r#"Application state for the Axum router.

Holds shared components that extractors rely on:
- [`Db`] — SQLx pool
- [`Key`] — cookie crypto key for [`PrivateCookieJar`]

Implements `FromRef` for `Db` and `Key` so handlers can extract them via `State<Db>` and extractors like `PrivateCookieJar`.

# Example

```rust,no_run
# use axum::Router;
# use axum_extra::extract::cookie::Key;
# async fn demo(db: sleep_api::db::Db) {
let state = sleep_api::app::AppState { db, key: sleep_api::config::session_key() };
let app: Router<sleep_api::app::AppState> = Router::new().with_state(state);
# }
```

[`Db`]: crate::db::Db
[`Key`]: axum_extra::extract::cookie::Key
[`PrivateCookieJar`]: axum_extra::extract::cookie::PrivateCookieJar
"#]
pub struct AppState {
    pub db: Db,
    pub key: Key,
}

impl axum::extract::FromRef<AppState> for Db {
    fn from_ref(s: &AppState) -> Db {
        s.db.clone()
    }
}

impl axum::extract::FromRef<AppState> for Key {
    fn from_ref(s: &AppState) -> Key {
        s.key.clone()
    }
}

pub fn router(db: Db) -> Router {
    let key: Key = crate::config::session_key();
    let enable_hsts = crate::config::hsts_enabled();

    let state = AppState {
        db,
        key: key.clone(),
    };
    let router = Router::new()
        .route("/", get(root))
        .route("/health", get(health_get).head(health_head))
        .route("/login", post(post_login))
        .route("/login.json", post(post_login_json))
        .route("/logout", post(post_logout))
        .route("/api/session", get(api_session))
        .route("/sleep", post(create_sleep))
        .route("/sleep/date/{date}", get(get_sleep))
        .route("/sleep/{id}", put(update_sleep).delete(delete_sleep))
        .route("/exercise", post(create_exercise))
        .route("/note", post(create_note))
        .route("/api/trends/sleep-bars", get(trends::sleep_bars))
        .route("/api/trends/summary", get(trends::summary))
        .with_state(state);

    crate::security::headers::apply(router, enable_hsts)
}

// Health endpoints for SvelteKit UI
async fn health_get() -> Json<serde_json::Value> {
    Json(json!({"status":"ok"}))
}
async fn health_head() -> StatusCode {
    StatusCode::OK
}

// Session probe for UI
async fn api_session(jar: PrivateCookieJar) -> Json<serde_json::Value> {
    let authed = current_user_from_cookie(&jar).is_some();
    Json(json!({"authenticated": authed}))
}

#[doc = r#"Redirect root to /trends.

Security:
- Requires an authenticated session (via [`RequireSessionRedirect`]). Unauthenticated users are redirected to `/login`.

Responses:
- 303 See Other — redirects to `/trends`

See also: [`trends_page`], [`crate::middleware::auth_layer::RequireSessionRedirect`]
"#]
async fn root() -> StatusCode {
    StatusCode::NO_CONTENT
}


#[doc = r#"Login (form) and issue session + CSRF cookies.

Accepts: `POST /login` (`application/x-www-form-urlencoded`)
- Body: `{ email, password }`
- On success:
  - Issues encrypted session cookie (see [`crate::config::session_cookie_name`])
  - Issues CSRF cookie (see [`crate::config::csrf_cookie_name`])
  - Redirects to `/`

Security:
- Verifies credentials against `ADMIN_EMAIL` + `ADMIN_PASSWORD_HASH`
- Cookie names/flags vary with `COOKIE_SECURE`; see [`crate::config::session_cookie_name`] / [`crate::config::csrf_cookie_name`]

Responses:
- 303 See Other — on success (redirect to `/`)
- 401 Unauthorized — on invalid credentials (HTML body)

Example:
```bash
curl -i -X POST http://localhost:8080/login \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'email=admin@example.com&password=...' \
  -c cookies.txt
```

See also: [`crate::auth::{verify_login, create_session_cookie}`], [`crate::security::csrf::issue_csrf_cookie`]
"#]
async fn post_login(
    jar: PrivateCookieJar,
    Form(creds): Form<LoginPayload>,
) -> axum::response::Response {
    if auth::verify_login(&creds.email, &creds.password) {
        let jar = auth::create_session_cookie(jar, "admin");
        let jar = jar.add(issue_csrf_cookie());
        (jar, Redirect::to("/")).into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Html("Invalid credentials".to_string()),
        )
            .into_response()
    }
}

#[doc = r#"Login (JSON) and issue session + CSRF cookies.

Accepts: `POST /login.json` (`application/json`)
- Body: `{ "email": "...", "password": "..." }`
- On success: `{"ok": true}` and `Set-Cookie` headers for session + CSRF

Responses:
- 200 OK — on success
- 401 Unauthorized — `{"error":"unauthorized"}`

Note:
- JSON route is functionally equivalent to the form `/login`. Prefer `/login` for browser-based flows.

Example:
```bash
curl -i -X POST http://localhost:8080/login.json \
  -H 'Content-Type: application/json' \
  -d '{"email":"admin@example.com","password":"..."}' \
  -c cookies.txt
```

See also: [`crate::auth::{verify_login, create_session_cookie}`], [`crate::security::csrf::issue_csrf_cookie`]
"#]
async fn post_login_json(
    jar: PrivateCookieJar,
    Json(creds): Json<LoginPayload>,
) -> axum::response::Response {
    if auth::verify_login(&creds.email, &creds.password) {
        let jar = auth::create_session_cookie(jar, "admin");
        let jar = jar.add(issue_csrf_cookie());
        (jar, Json(json!({"ok": true}))).into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error":"unauthorized"})),
        )
            .into_response()
    }
}

#[doc = r#"Logout and clear cookies.

Accepts: `POST /logout`

Security:
- Requires a valid CSRF header (double-submit) via [`CsrfGuard`]
- Session is cleared if present; operation is idempotent

Responses:
- 204 No Content — session + CSRF cookies cleared

Example:
```bash
curl -i -X POST http://localhost:8080/logout \
  -H "Cookie: __Host-session=...; __Host-csrf=..." \
  -H "X-CSRF-Token: <csrf cookie value>"
```

See also: [`crate::auth::clear_session_cookie`], [`crate::security::csrf::CsrfGuard`]
"#]
async fn post_logout(mut jar: PrivateCookieJar, _csrf: CsrfGuard) -> axum::response::Response {
    jar = auth::clear_session_cookie(jar);
    let csrf = Cookie::build((crate::config::csrf_cookie_name(), String::new()))
        .path("/")
        .secure(crate::config::cookie_secure())
        .http_only(false)
        .same_site(SameSite::Lax)
        .build();
    jar = jar.remove(csrf);
    (jar, StatusCode::NO_CONTENT).into_response()
}

#[doc = r#"Create a sleep session.

Accepts: `POST /sleep` (`application/json`)
- Body: [`SleepInput`]

Security:
- Requires authenticated session ([`RequireSessionJson`])
- Requires CSRF header equal to CSRF cookie ([`CsrfGuard`])

Responses:
- 201 Created — `{"id": <number>}`
- 401 Unauthorized — no/invalid session
- 403 Forbidden — CSRF failure

Example:
```bash
curl -i -X POST http://localhost:8080/sleep \
  -H "Cookie: __Host-session=...; __Host-csrf=..." \
  -H "X-CSRF-Token: <csrf cookie value>" \
  -H "Content-Type: application/json" \
  -d '{"date":"2025-06-17","bed_time":"22:05:00","wake_time":"06:30:00","latency_min":10,"awakenings":0,"quality":4}'
```

See also: [`crate::handlers::create_sleep`], [`crate::middleware::auth_layer::RequireSessionJson`], [`crate::security::csrf::CsrfGuard`]
"#]
async fn create_sleep(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(input): Json<SleepInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_sleep(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}

#[doc = r#"Get a sleep session for a wake date.

Accepts: `GET /sleep/date/{date}`
- Path param `date`: `YYYY-MM-DD` (wake date)

Security:
- Requires authenticated session ([`RequireSessionJson`])

Responses:
- 200 OK — [`SleepSession`]
- 401 Unauthorized — no/invalid session
- 404 Not Found — no entry for date

See also: [`crate::handlers::get_sleep_by_date`]
"#]
async fn get_sleep(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    Path(date): Path<chrono::NaiveDate>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    match handlers::get_sleep_by_date(&db, date).await? {
        Some(s) => Ok(Json(s)),
        None => Err(ApiError::NotFound),
    }
}

#[doc = r#"Update a sleep session by id.

Accepts: `PUT /sleep/{id}` (`application/json`)
- Body: [`SleepInput`]

Security:
- Requires authenticated session ([`RequireSessionJson`])
- Requires CSRF ([`CsrfGuard`])

Responses:
- 204 No Content — updated
- 401 Unauthorized — no/invalid session
- 403 Forbidden — CSRF failure

See also: [`crate::handlers::update_sleep`]
"#]
async fn update_sleep(
    State(db): State<Db>,
    Path(id): Path<i64>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(input): Json<SleepInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    handlers::update_sleep(&db, id, input).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[doc = r#"Delete a sleep session by id.

Accepts: `DELETE /sleep/{id}`

Security:
- Requires authenticated session ([`RequireSessionJson`])
- Requires CSRF ([`CsrfGuard`])

Responses:
- 204 No Content — deleted or already absent
- 401 Unauthorized — no/invalid session
- 403 Forbidden — CSRF failure

See also: [`crate::handlers::delete_sleep`]
"#]
async fn delete_sleep(
    State(db): State<Db>,
    Path(id): Path<i64>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let affected = handlers::delete_sleep(&db, id).await?;
    if affected == 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

#[doc = r#"Create an exercise entry.

Accepts: `POST /exercise` (`application/json`)
- Body: [`ExerciseInput`]

Security:
- Requires authenticated session ([`RequireSessionJson`])
- Requires CSRF ([`CsrfGuard`])

Responses:
- 201 Created — `{"id": <number>}`
- 401 Unauthorized
- 403 Forbidden — CSRF failure

See also: [`crate::handlers::create_exercise`]
"#]
async fn create_exercise(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(input): Json<ExerciseInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_exercise(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}

#[doc = r#"Create a note.

Accepts: `POST /note` (`application/json`)
- Body: [`NoteInput`]

Security:
- Requires authenticated session ([`RequireSessionJson`])
- Requires CSRF ([`CsrfGuard`])

Responses:
- 201 Created — `{"id": <number>}`
- 401 Unauthorized
- 403 Forbidden — CSRF failure

See also: [`crate::handlers::create_note`]
"#]
async fn create_note(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(input): Json<NoteInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_note(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}
