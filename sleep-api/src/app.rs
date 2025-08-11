#![doc = r#"HTTP routing

Defines the Axum [`Router`] that exposes the SleepTracker API. This module wires
all HTTP routes (health check, sleep CRUD, exercise, notes, and trends).

See the OpenAPI specification for request/response details:
- <https://github.com/ebigunso/SleepTracker/blob/main/openapi.yaml>

For an end-to-end server setup example, see [`router`].

[`Router`]: axum::Router
"#]

use crate::auth::{self, LoginPayload};
use crate::middleware::auth_layer::{RequireSessionJson, RequireSessionRedirect};
use crate::security::csrf::{CSRF_COOKIE, CsrfGuard, issue_csrf_cookie};
use crate::{
    db::Db,
    error::ApiError,
    handlers,
    models::{ExerciseInput, NoteInput, SleepInput},
    trends,
};
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::{
    Json, Router,
    extract::{Either, Form, Path, State},
    routing::{get, post, put},
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
        .route("/health", get(|| async { Json(json!({"status":"ok"})) }))
        .route("/login", get(get_login).post(post_login))
        .route("/logout", post(post_logout))
        .route("/sleep", post(create_sleep))
        .route("/sleep/date/{date}", get(get_sleep))
        .route("/sleep/{id}", put(update_sleep).delete(delete_sleep))
        .route("/exercise", post(create_exercise))
        .route("/note", post(create_note))
        .route("/api/trends/sleep-bars", get(trends::sleep_bars))
        .route("/api/trends/summary", get(trends::summary))
        .route("/trends", get(trends_page))
        .with_state(state);

    crate::security::headers::apply(router, enable_hsts)
}

async fn root(RequireSessionRedirect { _user_id: _ }: RequireSessionRedirect) -> Redirect {
    Redirect::to("/trends")
}

async fn get_login() -> Html<String> {
    let html = r#"<!doctype html>
<html>
<head><meta charset="utf-8"><title>Login</title></head>
<body>
  <h1>Login</h1>
  <form method="post" action="/login">
    <label>Email <input type="email" name="email" /></label><br/>
    <label>Password <input type="password" name="password" /></label><br/>
    <button type="submit">Login</button>
  </form>
</body>
</html>"#;
    Html(html.to_string())
}

async fn post_login(
    jar: PrivateCookieJar,
    payload: Either<Form<LoginPayload>, Json<LoginPayload>>,
) -> axum::response::Response {
    let creds = match payload {
        Either::Left(Form(c)) => c,
        Either::Right(Json(c)) => c,
    };
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

async fn post_logout(mut jar: PrivateCookieJar) -> axum::response::Response {
    jar = auth::clear_session_cookie(jar);
    let csrf = Cookie::build((CSRF_COOKIE, String::new()))
        .path("/")
        .secure(true)
        .http_only(false)
        .same_site(SameSite::Lax)
        .build();
    jar = jar.remove(csrf);
    (jar, StatusCode::NO_CONTENT).into_response()
}

async fn create_sleep(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(input): Json<SleepInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_sleep(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}

async fn get_sleep(
    State(db): State<Db>,
    Path(date): Path<chrono::NaiveDate>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    match handlers::get_sleep_by_date(&db, date).await? {
        Some(s) => Ok(Json(s)),
        None => Err(ApiError::NotFound),
    }
}

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

async fn create_exercise(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(input): Json<ExerciseInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_exercise(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}

async fn create_note(
    State(db): State<Db>,
    RequireSessionJson { _user_id: _ }: RequireSessionJson,
    _csrf: CsrfGuard,
    Json(input): Json<NoteInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_note(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}

async fn trends_page(
    RequireSessionRedirect { _user_id: _ }: RequireSessionRedirect,
) -> Html<String> {
    let tpl = super::views::TrendsTemplate;
    match tpl.render() {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Template rendering error: {}", e);
            Html("An internal error occurred while rendering the page.".to_string())
        }
    }
}
