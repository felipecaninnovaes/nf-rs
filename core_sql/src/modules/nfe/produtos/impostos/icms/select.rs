#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::IcmsId;
use sqlx::Row;
use std::error::Error;

pub async fn select_icms_id(
    pool: &sqlx::PgPool,
    idproduto: &i32,
) -> Result<IcmsId, Box<dyn Error>> {
    let q = "SELECT icms_idicms FROM nfe_icms WHERE icms_idproduto = $1";
    let result: IcmsId = sqlx::query_as::<_, IcmsId>(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?;
    Ok(result)
}
