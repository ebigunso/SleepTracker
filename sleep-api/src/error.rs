use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("not found")]
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Db(e) => {
                error!(?e, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":"db error"})),
                )
                    .into_response()
            }
            ApiError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({"error":"not found"}))).into_response()
            }
        }
    }
}
