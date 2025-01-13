use anyhow::Error;
use axum::http::HeaderMap;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sqlx::SqlitePool;
pub async fn authentication(
    header: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(Authorization(bearer)) = header.typed_get::<Authorization<Bearer>>() {
        if bearer.token().eq("fuck") {
            Ok(next.run(request).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
#[derive(Clone)]
pub struct WebApp {
    db: SqlitePool,
}
impl WebApp {
    pub async fn new(db_url: &str) -> Result<Self, Error> {
        Ok(Self {
            db: SqlitePool::connect(db_url).await?,
        })
    }
    pub async fn db(&self) -> &SqlitePool {
        &self.db
    }
}
pub struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Oh,shit:{}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
