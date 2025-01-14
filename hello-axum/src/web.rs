use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;
pub mod auth;
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
