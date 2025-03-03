use anyhow::anyhow;
// use anyhow::{Ok};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{Local, NaiveDateTime},
    SqlitePool,
};
use std::sync::Arc;

use crate::web::{AppError, WebApp};
pub fn router() -> Router<Arc<WebApp>> {
    Router::new()
        .route("/user", post(add_user))
        .route("/user/{name}", get(get_user))
}

#[derive(Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    name: String,
    password: String,
    solt: Vec<u8>,
    status: UserStatus,
    certificate: Vec<u8>,
    create_by: String,
    create_at: NaiveDateTime,
}
impl User {
    pub fn name(&self) -> &str {
        &self.name
    }
}
#[derive(Serialize, Deserialize, sqlx::Type)]
#[repr(i64)]
pub(crate) enum UserStatus {
    Active = 0,
    InActive = 1,
    UnKnow = 2,
}
impl From<i64> for UserStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Active,
            1 => Self::InActive,
            _ => Self::UnKnow,
        }
    }
}
#[derive(Deserialize)]
pub(crate) struct CreateUserRequest {
    name: String,
    password: String,
    certificate: Vec<u8>,
}
pub async fn add_user(
    State(app): State<Arc<WebApp>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(), AppError> {
    let mut conn = app.db().await.acquire().await?;
    let user = User {
        name: request.name,
        password: request.password,
        solt: vec![1],
        certificate: request.certificate,
        status: UserStatus::Active,
        create_by: "".to_string(),
        create_at: Local::now().naive_utc(),
    };
    sqlx::query!(
        "
        insert into user
        (name,
        password,
        solt,
        certificate,
        status,
        create_by,
        create_at)
        values (?1,?2,?3,?4,?5,?6,?7)
        ",
        user.name,
        user.password,
        user.solt,
        user.certificate,
        user.status,
        user.create_by,
        user.create_at,
    )
    .execute(&mut *conn)
    .await?;
    Ok(())
}
pub async fn get_user(
    State(app): State<Arc<WebApp>>,
    Path(name): Path<String>,
) -> Result<Json<User>, AppError> {
    Ok(Json(load_user_by_name(app.db().await, &name).await?))
}
pub(crate) async fn load_user_by_name(pool: &SqlitePool, name: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        "
        SELECT name,
        password,
        solt,
        certificate,
        status,
        create_by,
        create_at
        FROM user where name = ?1
        ",
        name
    )
    .fetch_one(pool)
    .await
    .map_err(|_| anyhow!("用户不存在"))?;
    Ok(user)
}
