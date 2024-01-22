#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::produtos::ProdutoId;
use sqlx::Row;
use std::error::Error;

pub async fn select_idproduto(
    pool: &sqlx::PgPool,
    nitem: &String,
    idnfe: &i32,
) -> Result<ProdutoId, Box<dyn Error>> {
    let q = "SELECT produto_idproduto FROM nfe_produto WHERE produto_nitem = $1 AND produto_idnfe = $2";
    // let idnfe_i32 = idnfe.parse::<i32>().unwrap();
    let idproduto = sqlx::query_as::<_, ProdutoId>(q)
        .bind(nitem)
        .bind(idnfe)
        .fetch_one(pool)
        .await?;
    Ok(idproduto)
}