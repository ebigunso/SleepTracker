use crate::domain::DomainError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("not found")]
    NotFound,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Db(e) => {
                error!(?e, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":"database error","detail": e.to_string()})),
                )
                    .into_response()
            }
            ApiError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({"error":"not found"}))).into_response()
            }
            ApiError::InvalidInput(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"invalid input","detail": msg})),
            )
                .into_response(),
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        ApiError::InvalidInput(err.to_string())
    }
}
