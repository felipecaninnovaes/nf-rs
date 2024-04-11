use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::{
    modules::utils::create::{create_id_and_current_date, CreateIdAndCurrentDateModel},
    structs::empresas::empresa_struct::CreateEmpresasModel,
};

#[allow(dead_code)]
pub async fn insert_empresa(
    pool: &PgPool,
    empresa: CreateEmpresasModel,
) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
    let id_and_current_date: CreateIdAndCurrentDateModel = create_id_and_current_date();

    let result = sqlx::query!(
        r#"INSERT INTO empresas (idempresa, nome, nome_fant, cnpj, rua, numero, bairro, cidade, estado, cep, telefone, email, regime_tributario, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
        id_and_current_date.id,
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
        id_and_current_date.current_date,
    )
    .execute(pool)
    .await?;

    match result.rows_affected() {
        1 => Ok(id_and_current_date.id),
        _ => Err("Failed to insert user".into()),
    }
}
