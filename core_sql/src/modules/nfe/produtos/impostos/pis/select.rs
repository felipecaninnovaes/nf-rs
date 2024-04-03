#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::PisId;
use sqlx::Row;
use std::error::Error;

pub async fn select_pis_id(pool: &sqlx::PgPool, idproduto: &i32) -> Result<PisId, Box<dyn Error>> {
    let q = "SELECT pis_idpis FROM nfe_pis WHERE pis_idproduto = $1";
    let result: PisId = sqlx::query_as::<_, PisId>(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?;
    Ok(result)
}
