#![allow(unused_imports, dead_code)]
use nfse::modules::structs::{EnderecoID, PrestadorID, TomadorID};

pub async fn select_nfse_endereco(
    pool: &sqlx::PgPool,
    numero: &String,
    cep: &String,
) -> Option<EnderecoID> {
    let result = sqlx::query!(
        r#"SELECT id FROM nfse_endereco WHERE numero = $1 AND cep = $2"#,
        numero,
        cep
    )
    .fetch_one(pool)
    .await;
    match result {
        Ok(r) => Some(EnderecoID { endereco_id: r.id }),
        Err(_) => None,
    }
}

pub async fn select_nfse_tomador_id(
    pool: &sqlx::PgPool,
    cnpj: &String,
    cpf: &String,
) -> Option<TomadorID> {
    let result = sqlx::query!(
        r#"SELECT id FROM nfse_tomador WHERE cnpj = $1 AND cpf = $2"#,
        cnpj,
        cpf
    )
    .fetch_one(pool)
    .await;
    match result {
        Ok(r) => Some(TomadorID { tomador_id: r.id }),
        Err(_) => None,
    }
}

pub async fn select_nfse_prestador_id(pool: &sqlx::PgPool, cnpj: &String) -> Option<PrestadorID> {
    let result = sqlx::query!(r#"SELECT id FROM nfse_prestador WHERE cnpj = $1"#, cnpj)
        .fetch_one(pool)
        .await;
    match result {
        Ok(r) => Some(PrestadorID { prestador_id: r.id }),
        Err(_) => None,
    }
}
