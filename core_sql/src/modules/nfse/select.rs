#![allow(unused_imports, dead_code, unused_variables)]
use std::error::Error;

use nfse::structs::EnderecoId;

pub async fn select_endereco_id_from_cep_numero(
    pool: &sqlx::PgPool,
    cep: &str,
    numero: &str,
) -> Result<EnderecoId, Box<dyn Error>> {
    let result = sqlx::query!(
        r#"SELECT id FROM endereco WHERE cep = $1 AND numero = $2"#,
        cep,
        numero,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    Ok(EnderecoId {
        endereco_id: result.id,
    })
}

pub async fn select_prestador_id_from_cnpj(
    pool: &sqlx::PgPool,
    cnpj: &str,
) -> Result<i32, Box<dyn Error>> {
    let result = sqlx::query!(r#"SELECT id FROM prestador WHERE cnpj = $1"#, cnpj,)
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(result.id)
}
