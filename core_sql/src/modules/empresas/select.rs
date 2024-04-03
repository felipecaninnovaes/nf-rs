use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::structs::empresas::empresa_struct::EmpresasGetModel;

#[allow(dead_code)]
pub async fn select_empresas_by_cnpj(
    pool: &PgPool,
    cnpj: &str,
) -> Result<EmpresasGetModel, Box<dyn Error + Send + Sync>> {
    let q = format!("SELECT * FROM empresas WHERE cnpj = '{}'", cnpj);
    match sqlx::query_as::<_, EmpresasGetModel>(&q)
        .fetch_one(pool)
        .await
    {
        Ok(empresas) => Ok(empresas),
        Err(_) => Err("Empresa não encontrada".into()),
    }
}

#[allow(dead_code)]
pub async fn select_empresas_by_id(
    pool: &PgPool,
    idempresa: &Uuid,
) -> Result<EmpresasGetModel, Box<dyn Error + Sync + Send>> {
    let q = format!("SELECT * FROM empresas WHERE idempresa = '{}'", idempresa);
    match sqlx::query_as::<_, EmpresasGetModel>(&q)
        .fetch_one(pool)
        .await
    {
        Ok(empresas) => Ok(empresas),
        Err(_) => Err("Empresa não encontrada".into()),
    }
}
