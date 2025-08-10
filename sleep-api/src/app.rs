#![doc = r#"HTTP routing

Defines the Axum [`Router`] that exposes the SleepTracker API. This module wires
all HTTP routes (health check, sleep CRUD, exercise, notes, and trends).

See the OpenAPI specification for request/response details:
- <https://github.com/ebigunso/SleepTracker/blob/main/openapi.yaml>

For an end-to-end server setup example, see [`router`].

[`Router`]: axum::Router
"#]

use crate::{
    db::Db,
    error::ApiError,
    handlers,
    models::{ExerciseInput, NoteInput, SleepInput},
    trends,
};
use askama::Template;
use axum::http::StatusCode;
use axum::response::Html;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post, put},
};
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
pub fn router(db: Db) -> Router {
    Router::new()
        .route("/health", get(|| async { Json(json!({"status":"ok"})) }))
        .route("/sleep", post(create_sleep))
        .route("/sleep/date/{date}", get(get_sleep))
        .route("/sleep/{id}", put(update_sleep).delete(delete_sleep))
        .route("/exercise", post(create_exercise))
        .route("/note", post(create_note))
        .route("/api/trends/sleep-bars", get(trends::sleep_bars))
        .route("/api/trends/summary", get(trends::summary))
        .route("/trends", get(trends_page))
        .with_state(db)
}

async fn create_sleep(
    State(db): State<Db>,
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
    Json(input): Json<SleepInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    handlers::update_sleep(&db, id, input).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_sleep(
    State(db): State<Db>,
    Path(id): Path<i64>,
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
    Json(input): Json<ExerciseInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_exercise(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}

async fn create_note(
    State(db): State<Db>,
    Json(input): Json<NoteInput>,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    let id = handlers::create_note(&db, input).await?;
    Ok((StatusCode::CREATED, Json(json!({"id": id}))))
}

async fn trends_page() -> Html<String> {
    let tpl = super::views::TrendsTemplate;
    match tpl.render() {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Template rendering error: {}", e);
            Html("An internal error occurred while rendering the page.".to_string())
        }
    }
}
