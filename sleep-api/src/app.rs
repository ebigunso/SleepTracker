use crate::{
    db::Db,
    error::ApiError,
    handlers,
    trends,
    models::{ExerciseInput, NoteInput, SleepInput},
};
use axum::http::StatusCode;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post, put},
};
use serde_json::json;
use axum::response::Html;
use askama::Template;

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
    Html(tpl.render().unwrap_or_else(|_| "Template error".to_string()))
}
