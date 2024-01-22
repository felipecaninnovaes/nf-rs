#![allow(dead_code, unused_variables, unused_imports)]
use std::error::Error;

use nfe::modules::json::structs::nfe_struct::{NfeId, NfeJoinSelect, NfeSelect};
use uuid::Uuid;

use crate::modules::usuarios::select::select_all_user_cnpj_permissoes;

pub async fn select_nfe_id(
    pool: &sqlx::PgPool,
    nnf: &String,
    idemit: &i32,
) -> Result<NfeId, Box<dyn Error>> {
    let q = "SELECT nfe_idnfe FROM nfe WHERE nfe_nnf = $1 AND nfe_idemit = $2";
    let query = sqlx::query_as::<_, NfeId>(q)
        .bind(nnf)
        .bind(idemit)
        .fetch_one(pool)
        .await?;
    match query.nfe_idnfe {
        0 => Err("nfe_idnfe not found".into()),
        _ => Ok(query),
    }
}

pub async fn all_nfe(pool: &sqlx::PgPool, user_id: Uuid) -> Result<String, Box<dyn Error>> {
    let vec_cnpjs = select_all_user_cnpj_permissoes(pool, user_id)
        .await
        .expect("Erro ao buscar CNPJs");

    let mut result: Vec<NfeJoinSelect> = Vec::new();

    for row in vec_cnpjs {
        let q = "SELECT * FROM nfe INNER JOIN nfe_emit ON nfe.nfe_idemit = nfe_emit.emit_idemit INNER JOIN nfe_dest ON nfe.nfe_iddest = nfe_dest.dest_iddest WHERE nfe_emit.emit_cnpjcpf = $1 or nfe_dest.dest_cnpjcpf = $1";
        let res = sqlx::query_as::<_, NfeJoinSelect>(q)
            .bind(&row.cnpj)
            .fetch_all(pool)
            .await?;
        for row in res {
            result.push(row);
        }
    }

    let json = serde_json::to_string(&result)?;
    Ok(json)
}

// get nfe by emit
pub async fn nfe_by_emit(pool: &sqlx::PgPool, emit: &String) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe INNER JOIN nfe_emit ON nfe.nfe_idemit = nfe_emit.emit_idemit INNER JOIN nfe_dest ON nfe.nfe_iddest = nfe_dest.dest_iddest WHERE nfe_emit.emit_cnpjcpf = $1";
    let res = sqlx::query_as::<_, NfeJoinSelect>(q)
        .bind(emit)
        .fetch_all(pool)
        .await?;
    let json = serde_json::to_string(&res)?;
    Ok(json)
}

// get nfe by dest
pub async fn nfe_by_dest(pool: &sqlx::PgPool, dest: &i32) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe WHERE nfe_iddest = $1";
    let res = sqlx::query_as::<_, NfeSelect>(q)
        .bind(dest)
        .fetch_all(pool)
        .await?;
    let json = serde_json::to_string(&res)?;
    Ok(json)
}
