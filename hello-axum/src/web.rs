use anyhow::Error;
use axum::{http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;
pub mod auth;
pub mod role;
pub mod user;
pub mod user_role;
#[derive(Clone)]
pub struct WebApp {
    web_db: SqlitePool,
}
impl WebApp {
    pub async fn new(web_db_url: &str) -> Result<Self, Error> {
        Ok(Self {
            web_db: SqlitePool::connect(web_db_url).await?,
        })
    }
    pub async fn db(&self) -> &SqlitePool {
        &self.web_db
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
