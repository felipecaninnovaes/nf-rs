use std::error::Error; // Add missing import for the Error trait
use sqlx::Row;

// get vec products id from a nfe
pub async fn get_products_id_from_nfe(pool: &sqlx::PgPool, nfeid: &i32) -> Result<Vec<i32>, Box<dyn Error>> {
    let q = "SELECT idproduto FROM produto WHERE nfeidnfe = $1";
    let mut v: Vec<i32> = Vec::new();
    for row in sqlx::query(q).bind(nfeid).fetch_all(pool).await? {
        v.push(row.get(0));
    }
    Ok(v)
}