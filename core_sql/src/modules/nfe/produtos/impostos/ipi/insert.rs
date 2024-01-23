#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::{Ipi, IpiId};
use sqlx::Row;
use std::error::Error;

use super::select::select_ipi_id;

pub async fn insert_ipi_sql(
    pool: &sqlx::PgPool,
    imposto: &Ipi,
    idproduto: &i32,
) -> Result<IpiId, Box<dyn Error>> {
    let q = "INSERT INTO nfe_ipi (ipi_cenq, ipi_cst, ipi_vbc, ipi_pipi, ipi_vipi, ipi_idproduto) VALUES ($1, $2, $3, $4, $5, $6) RETURNING ipi_idipi";
    match select_ipi_id(pool, idproduto).await {
        Ok(imposto_id) => Ok(imposto_id),
        Err(_) => {
            let result = sqlx::query(q)
                .bind(imposto.ipi_cenq)
                .bind(imposto.ipi_cst)
                .bind(imposto.ipi_vbc)
                .bind(imposto.ipi_pipi)
                .bind(imposto.ipi_vipi)
                .bind(idproduto)
                .fetch_one(pool)
                .await?
                .get::<i32, _>(0);
            Ok(IpiId { ipi_idipi: result })
        }
    }
}

