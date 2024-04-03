use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::structs::permissoes::struct_permissions::Permissions;

#[allow(dead_code)]
pub async fn select_user_permission_by_empresa_id(
    pool: &PgPool,
    user_id: Uuid,
    empresa_id: Uuid,
) -> Result<Permissions, Box<dyn Error + Send + Sync>> {
    let q =
        "SELECT * FROM permissions WHERE permissions_user_id = $1 and permissions_empresa_id = $2";
    match sqlx::query_as::<_, Permissions>(q)
        .bind(user_id)
        .bind(empresa_id)
        .fetch_one(pool)
        .await
    {
        Ok(permissions) => Ok(permissions),
        Err(_) => Err("Permissão não encontrada".into()),
    }
}

#[allow(dead_code)]
pub async fn select_all_user_permissions(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Permissions>, Box<dyn Error + Send + Sync>> {
    let allowed: bool = true;
    let q_empresas_allowed: &str = "SELECT permissions.permissions_user_id, permissions.permissions_allowed, empresas.cnpj from permissions INNER JOIN empresas ON permissions.permissions_empresa_id = empresas.idempresa where permissions.permissions_user_id = $1 and permissions.permissions_allowed = $2";

    match sqlx::query_as::<_, Permissions>(q_empresas_allowed)
        .bind(user_id)
        .bind(allowed)
        .fetch_all(pool)
        .await
    {
        Ok(permissions) => Ok(permissions),
        Err(_) => Err("Permissões não encontradas".into()),
    }
}
