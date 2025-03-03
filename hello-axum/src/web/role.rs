use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::web::AppError;
#[derive(Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
}
impl Role {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
pub(in crate::web) async fn load_role_by_id(pool: &SqlitePool, id: i64) -> Result<Role, AppError> {
    let user = sqlx::query_as!(
        Role,
        "
        SELECT id,
        name
        FROM role where id = ?1
        ",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| anyhow!("角色不存在"))?;
    Ok(user)
}
