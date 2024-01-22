#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::{Pis, PisId};
use sqlx::Row;
use std::error::Error;

use super::select::select_pis_id;

pub async fn insert_pis_sql(
    pool: &sqlx::PgPool,
    imposto: &Pis,
    idproduto: &i32,
) -> Result<PisId, Box<dyn Error>> {
    let q = "INSERT INTO nfe_pis (pis_cst, pis_vbc, pis_ppis, pis_vpis, pis_idproduto) VALUES ($1, $2, $3, $4, $5) RETURNING pis_idpis";
    match select_pis_id(pool, idproduto).await {
        Ok(imposto_id) => Ok(imposto_id),
        Err(_) => {
            let result = sqlx::query(q)
                .bind(imposto.pis_cst)
                .bind(imposto.pis_vbc)
                .bind(imposto.pis_ppis)
                .bind(imposto.pis_vpis)
                .bind(idproduto)
                .fetch_one(pool)
                .await?
                .get::<i32, _>(0);
            Ok(PisId { pis_idpis: result })
        }
    }
}

