use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::web::{AppError, WebApp};

#[derive(Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}
fn router() -> Router {
    Router::new().route("/user", post(add_user))
}
async fn add_user(
    Json(user): Json<User>,
    // State(app): State<Arc<WebApp>>,
) -> Result<usize, AppError> {
    Ok(1)
}
// async fn add_user(
//     Json(user): Json<User>,
//     State(app): State<Arc<WebApp>>,
// ) -> Result<usize, AppError> {
//     let mut conn = app.db().await.acquire().await?;
//     let id = sqlx::query!(
//         r#"
//         insert into user (id,name) valus (?1,?2)
//         "#,
//         user.id,
//         user.name
//     )
//     .execute(&mut conn)
//     .await?
//     .last_insert_rowid();
//     Ok(id)
// }
