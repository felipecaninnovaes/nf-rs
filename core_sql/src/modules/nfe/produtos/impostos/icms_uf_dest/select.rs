#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::IcmsUfDestId;
use sqlx::Row;
use std::error::Error;

pub async fn select_icms_uf_dest_id(
    pool: &sqlx::PgPool,
    idproduto: &i32,
) -> Result<IcmsUfDestId, Box<dyn Error>> {
    let q = "SELECT id FROM nfe_icmsufdest WHERE icms_uf_idproduto = $1";
    let result: IcmsUfDestId = sqlx::query_as::<_, IcmsUfDestId>(q)
        .bind(idproduto)
        .fetch_one(pool)
        .await?;
    Ok(result)
}
