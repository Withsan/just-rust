use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::web::AppError;

use super::role::Role;
#[derive(Serialize, Deserialize)]
pub struct UserRole {
    id: i64,
    name: String,
}
impl UserRole {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
pub(in crate::web) async fn load_user_role(
    pool: &SqlitePool,
    name: &str,
) -> Result<Vec<Role>, AppError> {
    let users = sqlx::query_as!(
        Role,
        "
        SELECT
        r.id as id,
        r.name as name
        FROM role r,user_role ur  where r.id =ur.role_id and ur.user_name  = ?1
        ",
        name
    )
    .fetch_all(pool)
    .await
    .map_err(|_| anyhow!("用户不存在"))?;
    Ok(users)
}
