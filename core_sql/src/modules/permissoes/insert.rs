use std::error::Error;

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::modules::{
    empresas::select::select_empresas_by_id,
    utils::create::{create_id_and_current_date, CreateIdAndCurrentDateModel},
};

use super::select::select_all_user_permissions;

pub async fn insert_query(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    empresa_id: Uuid,
    allowed: bool,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let id_and_current_date: CreateIdAndCurrentDateModel = create_id_and_current_date();
    let result = sqlx::query!(
        r#"INSERT INTO permissions (permissions_idpermission, permissions_user_id, permissions_empresa_id, permissions_allowed, permissions_created_at) VALUES ($1, $2, $3, $4, $5)"#,
        id_and_current_date.id,
        user_id,
        empresa_id,
        allowed,
        id_and_current_date.current_date,
    ).execute(pool).await?;

    match result.rows_affected() {
        1 => Ok(()),
        _ => Err("Failed to insert user".into()),
    }
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
