use std::error::Error;

use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::select::select_all_user_permissions;

#[allow(dead_code)]
pub async fn insert_permission(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    empresa_id: Uuid,
    allowed: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let uuid = Uuid::new_v4();
    let _result = select_all_user_permissions(pool, user_id).await;

    if let Ok(result) = _result {
        if result.is_empty() {
            let created_at = Utc::now().naive_utc();
            let q = format!("INSERT INTO permissions (permissions_idpermission, permissions_user_id, permissions_empresa_id, permissions_allowed, permissions_created_at) VALUES ('{}', '{}', '{}', '{}', '{}')" , uuid, user_id, empresa_id, allowed, created_at);
            sqlx::query(&q).execute(pool).await.unwrap();
        }
    }
    Ok(())
}
