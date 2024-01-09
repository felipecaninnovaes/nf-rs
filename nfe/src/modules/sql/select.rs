use crate::modules::json::structs::nfe::{NfeSelect, NfeJoinSelect};
use sqlx::Row;
use std::error::Error;

// get vec products id from a nfe
pub async fn get_products_id_from_nfe(
    pool: &sqlx::PgPool,
    nfeid: &i32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let q = "SELECT produto_idproduto FROM nfe_produto WHERE produto_idnfe = $1";
    let mut v: Vec<i32> = Vec::new();
    for row in sqlx::query(q).bind(nfeid).fetch_all(pool).await? {
        v.push(row.get(0));
    }
    Ok(v)
}

// get all nfe
pub async fn all_nfe(pool: &sqlx::PgPool) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe INNER JOIN nfe_emit ON nfe.nfe_idemit = nfe_emit.emit_idemit INNER JOIN nfe_dest ON nfe.nfe_iddest = nfe_dest.dest_iddest";
    let res = sqlx::query_as::<_, NfeJoinSelect>(q).fetch_all(pool).await?;
    let json = serde_json::to_string(&res)?;
    Ok(json)
}

// get nfe by emit
pub async fn nfe_by_emit(pool: &sqlx::PgPool, emit: &String) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe INNER JOIN nfe_emit ON nfe.nfe_idemit = nfe_emit.emit_idemit INNER JOIN nfe_dest ON nfe.nfe_iddest = nfe_dest.dest_iddest WHERE nfe_emit.emit_cnpjcpf = $1";
    let res = sqlx::query_as::<_, NfeJoinSelect>(q)
        .bind(emit)
        .fetch_all(pool)
        .await?; // for row in sqlx::query(q).bind(emit).fetch_all(pool).await? {
    let json = serde_json::to_string(&res)?;
    Ok(json)
}

// get nfe by dest
pub async fn nfe_by_dest(pool: &sqlx::PgPool, dest: &i32) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe WHERE nfe_iddest = $1";
    let res = sqlx::query_as::<_, NfeSelect>(q)
        .bind(dest)
        .fetch_all(pool)
        .await?; // for row in sqlx::query(q).bind(dest).fetch_all(pool).await? {
    let json = serde_json::to_string(&res)?;
    Ok(json)
}
