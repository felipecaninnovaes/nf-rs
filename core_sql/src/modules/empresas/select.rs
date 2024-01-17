use sqlx::PgPool;
use std::error::Error;

use crate::structs::empresas::empresa_struct::EmpresasGetModel;

#[allow(dead_code)]
pub async fn select_all_empresas_by_cnpj(
    pool: &PgPool,
    cnpj: &str,
) -> Result<Vec<EmpresasGetModel>, Box<dyn Error>> {
    let q = format!("SELECT * FROM empresas WHERE cnpj = '{}'", cnpj);
    match sqlx::query_as::<_, EmpresasGetModel>(&q).fetch_all(pool).await {
        Ok(empresas) => Ok(empresas),
        Err(_) => Err("Empresa não encontrada".into()),
    }
}
