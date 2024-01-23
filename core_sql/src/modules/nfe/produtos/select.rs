#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::produtos::ProdutoId;
use sqlx::Row;
use std::error::Error;

pub async fn select_produto_id(
    pool: &sqlx::PgPool,
    nitem: &String,
    idnfe: &i32,
) -> Result<ProdutoId, Box<dyn Error>> {
    let q =
        "SELECT produto_idproduto FROM nfe_produto WHERE produto_nitem = $1 AND produto_idnfe = $2";
    let result = sqlx::query_as::<_, ProdutoId>(q)
        .bind(nitem)
        .bind(idnfe)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn select_products_id_from_nfe(
    pool: &sqlx::PgPool,
    nfeid: &i32,
) -> Result<Vec<ProdutoId>, Box<dyn Error>> {
    let q = "SELECT produto_idproduto FROM nfe_produto WHERE produto_idnfe = $1";
    let result = sqlx::query_as::<_, ProdutoId>(q)
        .bind(nfeid)
        .fetch_all(pool)
        .await?;
    Ok(result)
}
