use chrono::Utc;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::structs::empresas::empresa_struct::CreateEmpresasModel;

#[allow(dead_code)]
pub async fn insert_empresa(
    pool: &PgPool,
    empresa: CreateEmpresasModel,
) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc().date();

    let result = sqlx::query!(
        r#"INSERT INTO empresas (idempresa, nome, nome_fant, cnpj, rua, numero, bairro, cidade, estado, cep, telefone, email, regime_tributario, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
        uuid,
        empresa.nome.to_uppercase(),
        empresa.nome_fant.to_uppercase(),
        empresa.cnpj,
        empresa.rua.to_uppercase(),
        empresa.numero,
        empresa.bairro.to_uppercase(),
        empresa.cidade.to_uppercase(),
        empresa.estado.to_uppercase(),
        empresa.cep,
        empresa.telefone,
        empresa.email,
        empresa.regime_tributario.to_uppercase(),
        created_at
    )
    .execute(pool)
    .await?;

    match result.rows_affected() {
        1 => Ok(uuid),
        _ => Err("Failed to insert user".into()),
    }
}
