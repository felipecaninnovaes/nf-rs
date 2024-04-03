#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::IpiId;
use sqlx::Row;
use std::error::Error;

pub async fn select_ipi_id(pool: &sqlx::PgPool, idproduto: &i32) -> Result<IpiId, Box<dyn Error>> {
    let q = "SELECT ipi_idipi FROM nfe_ipi WHERE ipi_idproduto = $1";
    let result: IpiId = sqlx::query_as::<_, IpiId>(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?;
    Ok(result)
}
