use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local};
use std::sync::Arc;
use uuid::Uuid;

use crate::web::{AppError, WebApp};
pub fn router() -> Router<Arc<WebApp>> {
    Router::new()
        .route("/user", post(add_user))
        .route("/user/{id}", get(get_user))
}

#[derive(Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    id: String,
    name: String,
    password: String,
    solt: Vec<u8>,
    status: UserStatus,
    certificate: Vec<u8>,
    create_by: String,
    create_at: DateTime<Local>,
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
    solt: Vec<u8>,
    status: UserStatus,
    certificate: Vec<u8>,
}
pub async fn add_user(
    State(app): State<Arc<WebApp>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<i64>, AppError> {
    let mut conn = app.db().await.acquire().await?;
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: request.name,
        password: request.password,
        solt: vec![1],
        status: UserStatus::Active,
        certificate: request.certificate,
        create_by: "".to_string(),
        create_at: Local::now(),
    };
    let id = sqlx::query!(
        "
        insert into user
        (id,
        name,
        password,
        solt,
        status,
        certificate,
        create_by,
        create_at)
        values (?1,?2,?3,?4,?5,?6,?7,?8)
        ",
        user.id,
        user.name,
        user.password,
        user.solt,
        user.status,
        user.certificate,
        user.create_by,
        user.create_at,
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();
    Ok(Json::from(id))
}
pub async fn get_user(
    State(app): State<Arc<WebApp>>,
    Path(id): Path<String>,
) -> Result<Json<User>, AppError> {
    let mut conn = app.db().await.acquire().await?;
    let user = sqlx::query_as!(
        User,
        "
        SELECT id,
        name,
        password,
        solt,
        status,
        certificate,
        create_by,
        create_at
        FROM user where id = ?1
        ",
        id
    )
    .fetch_one(&mut *conn)
    .await?;
    Ok(Json(user))
}
