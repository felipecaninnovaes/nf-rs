use sqlx::PgPool;
use std::error::Error;

use crate::structs::permissoes::struct_permissions::PermissionsUpdate;
// use uuid::Uuid;

#[allow(dead_code)]
pub async fn update_permissions(
    pool: &PgPool,
    permission: PermissionsUpdate,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"UPDATE permissions SET permissions_empresa_id  = $1, permissions_user_id = $2, permissions_allowed = $3 WHERE permissions_idpermission = $4"#,
        permission.permissions_empresa_id,
        permission.permissions_user_id,
        permission.permissions_allowed,
        permission.permissions_idpermission,
    )
    .execute(pool)
    .await?;

    // let q = format!("UPDATE permissions SET cnpj = '{}', permissions_user_id = '{}', permissions_allowed = '{}' WHERE id = '{}'", permission.cnpj, permission.permissions_user_id, permission.permissions_allowed, permission.permissions_idpermission);
    // sqlx::query(&q).execute(pool).await?;
    Ok(())
}
