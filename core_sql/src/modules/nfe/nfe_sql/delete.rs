#![allow(unused_imports)]
use sqlx::Row;
use std::error::Error;

#[allow(dead_code)]
pub async fn delete_nfe(pool: &sqlx::PgPool, nfe_idnfe: &i32) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe WHERE nfe_idnfe = $1";
    sqlx::query(q).bind(nfe_idnfe).execute(pool).await?;

    Ok(())
}
