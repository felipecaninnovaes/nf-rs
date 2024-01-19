use std::error::Error;

use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::modules::empresas::select::select_empresas_by_id;

use super::select::select_all_user_permissions;

pub async fn insert_query(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    empresa_id: Uuid,
    allowed: bool,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc();
    let q = format!("INSERT INTO permissions (permissions_idpermission, permissions_user_id, permissions_empresa_id, permissions_allowed, permissions_created_at) VALUES ('{}', '{}', '{}', '{}', '{}')" , uuid, user_id, empresa_id, allowed, created_at);
    sqlx::query(&q).execute(pool).await.unwrap();
    Ok(())
}

#[allow(dead_code)]
pub async fn insert_permission(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    empresa_id: Uuid,
    allowed: bool,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let cnpj = select_empresas_by_id(pool, &empresa_id).await;
    let user_permisson = select_all_user_permissions(pool, user_id).await;
    match cnpj {
        Ok(_) => {
            match user_permisson {
                Ok(_) => {
                    println!("Usuário já possui permissão");
                    insert_query(pool, user_id, empresa_id, allowed).await?;
                }
                Err(_) => {
                    println!("Usuário não possui permissão");
                    insert_query(pool, user_id, empresa_id, allowed).await?;
                }
            }
            Ok(())
        }
        Err(_) => Err("Empresa não cadastrada".into()),
    }
}
