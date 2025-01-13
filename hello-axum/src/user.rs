use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::web::{AppError, WebApp};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
}
pub fn router() -> Router<Arc<WebApp>> {
    Router::new()
        .route("/user", post(add_user))
        .route("/user/{id}", get(get_user))
}
pub async fn add_user(
    State(app): State<Arc<WebApp>>,
    Json(user): Json<User>,
) -> Result<Json<i64>, AppError> {
    let mut conn = app.db().await.acquire().await?;
    let id = sqlx::query!(
        "
        insert into user (id,name) values (?1,?2)
        ",
        user.id,
        user.name
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();
    Ok(Json::from(id))
}
pub async fn get_user(
    State(app): State<Arc<WebApp>>,
    Path(id): Path<i64>,
) -> Result<Json<User>, AppError> {
    let mut conn = app.db().await.acquire().await?;
    let user = sqlx::query_as!(
        User,
        "
    SELECT id,name FROM user where id = ?1
    ",
        id
    )
    .fetch_one(&mut *conn)
    .await?;
    Ok(Json(user))
}
