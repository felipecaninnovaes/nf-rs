#![allow(dead_code, unused_variables, unused_imports)]
use crate::modules::json::structs::nfe_struct::{NfeJoinSelect, NfeSelect};
// use core_sql::modules::usuarios::select::select_all_user_cnpj_permissoes;
use sqlx::Row;
use std::error::Error;
use uuid::Uuid;

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