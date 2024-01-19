use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::{
    modules::empresas::select::select_empresas_by_id,
    structs::{
        empresas::empresa_struct::EmpresasCnpjModel,
        permissoes::struct_permissions::PermissionsEmpresaId,
        usuarios::struct_user::{UserSelectModel, UserSelectModelPassword},
    },
};

#[allow(dead_code)]
pub async fn select_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Vec<UserSelectModelPassword>, Box<dyn Error>> {
    let select_user = format!("SELECT * FROM users WHERE email = '{}'", email);
    match sqlx::query_as::<_, UserSelectModelPassword>(&select_user)
        .fetch_all(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Usuario não encontrado".into()),
    }
}

#[allow(dead_code)]
pub async fn select_user_by_id(
    pool: &PgPool,
    iduser: &str,
) -> Result<Vec<UserSelectModel>, Box<dyn Error>> {
    let select_user = format!("SELECT * FROM users WHERE iduser = '{}'", iduser);
    match sqlx::query_as::<_, UserSelectModel>(&select_user)
        .fetch_all(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Usuario não encontrado".into()),
    }
}

#[allow(dead_code)]
pub async fn select_all_users(pool: &PgPool) -> Result<Vec<UserSelectModel>, Box<dyn Error>> {
    let select_user = "SELECT * FROM users";
    match sqlx::query_as::<_, UserSelectModel>(select_user)
        .fetch_all(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Usuario não encontrado".into()),
    }
}

#[allow(dead_code)]
pub async fn select_all_user_cnpj_permissoes(
    pool: &PgPool,
    iduser: Uuid,
) -> Result<Vec<EmpresasCnpjModel>, Box<dyn Error>> {
    let empresas_id_vec =
        "SELECT permissions_empresa_id FROM permissions WHERE permissions_user_id = $1";
    let result: Vec<PermissionsEmpresaId> =
        sqlx::query_as::<_, PermissionsEmpresaId>(empresas_id_vec)
            .bind(iduser)
            .fetch_all(pool)
            .await?;
    let mut result_vec = Vec::new();
    for row in result {
        let empresa = select_empresas_by_id(pool, &row.permissions_empresa_id).await?;
        result_vec.push(EmpresasCnpjModel {
            cnpj: empresa.cnpj.clone(),
        });
    }
    Ok(result_vec)
}
