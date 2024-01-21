use std::error::Error;

use nfe::modules::json::structs::nfe_struct::NfeId;

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
