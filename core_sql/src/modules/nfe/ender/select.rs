use std::error::Error;

use nfe::modules::json::structs::ender::EnderId;

pub async fn select_ender_id(
    pool: &sqlx::PgPool,
    nro: &String,
    cep: &String,
) -> Result<EnderId, Box<dyn Error>> {
    let q = "SELECT ender_idender FROM nfe_ender WHERE ender_nro = $1 AND ender_cep = $2";
    let query = sqlx::query_as::<_, EnderId>(q)
        .bind(nro)
        .bind(cep)
        .fetch_one(pool)
        .await?;
    match query.ender_idender {
        0 => Err("emit_idemit not found".into()),
        _ => Ok(query),
    }
}
