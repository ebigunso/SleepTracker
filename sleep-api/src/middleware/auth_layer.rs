use axum::extract::{FromRef, FromRequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use serde_json::json;

use crate::auth::{UserId, current_user_from_cookie};

/// Extractor that requires an authenticated session for JSON APIs.
/// On failure, returns 401 with a JSON error payload.
pub struct RequireSessionJson {
    pub _user_id: UserId,
}

/// Extractor that requires an authenticated session for UI routes.
/// On failure, redirects to /login.
pub struct RequireSessionRedirect {
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

impl<S> FromRequestParts<S> for RequireSessionRedirect
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
            .map_err(|_| redirect_login())?;
        match current_user_from_cookie(&jar) {
            Some(uid) => Ok(Self { _user_id: uid }),
            None => Err(redirect_login()),
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

fn redirect_login() -> Response {
    Redirect::to("/login").into_response()
}
