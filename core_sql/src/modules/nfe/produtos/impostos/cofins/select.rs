#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::{Cofins, CofinsId};
use sqlx::Row;
use std::error::Error;

pub async fn select_cofins_id(
    pool: &sqlx::PgPool, idproduto: &i32
) -> Result<CofinsId, Box<dyn Error>> {
    let q = "SELECT cofins_idcofins FROM nfe_cofins WHERE cofins_idproduto = $1";
    // let idnfe_i32 = idnfe.parse::<i32>().unwrap();
    let result = sqlx::query_as::<_, CofinsId>(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?;
    Ok(result)
}