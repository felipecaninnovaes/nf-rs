use crate::modules::json::structs::nfe::NfeSelect;
use sqlx::Row;
use std::error::Error;

// get vec products id from a nfe
pub async fn get_products_id_from_nfe(
    pool: &sqlx::PgPool,
    nfeid: &i32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let q = "SELECT idproduto FROM produto WHERE nfeidnfe = $1";
    let mut v: Vec<i32> = Vec::new();
    for row in sqlx::query(q).bind(nfeid).fetch_all(pool).await? {
        v.push(row.get(0));
    }
    Ok(v)
}

// get all nfe
pub async fn all_nfe(pool: &sqlx::PgPool) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe INNER JOIN nfe_emit ON nfe.nfe_idemit = nfe_emit.nfe_idemit INNER JOIN nfe_dest ON nfe.nfe_iddest = nfe_dest.nfe_iddest";
    let res = sqlx::query_as::<_, NfeSelect>(q).fetch_all(pool).await?;
    let res2 = sqlx::query(q).fetch_all(pool).await?;
    println!("{:?}", res);
    let json = serde_json::to_string(&res)?;
    Ok(json)
}
// pub async fn all_nfe(pool: &sqlx::PgPool) -> Result<String, Box<dyn Error>> {
//     let q = "SELECT * FROM nfe";
//     let res = sqlx::query_as::<_, NfeSelect>(q).fetch_all(pool).await?;
//     let json = serde_json::to_string(&res)?;
//     Ok(json)
// }

// get nfe by emit
pub async fn nfe_by_emit(pool: &sqlx::PgPool, emit: &i32) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe WHERE nfe_idemit = $1";
    let res = sqlx::query_as::<_, NfeSelect>(q)
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
