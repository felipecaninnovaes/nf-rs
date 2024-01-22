#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::{Icms, IcmsId};
use sqlx::Row;
use std::error::Error;

use super::select::select_icms_id;

pub async fn insert_icms_sql(
    pool: &sqlx::PgPool,
    imposto: &Icms,
    idproduto: &i32,
) -> Result<IcmsId, Box<dyn Error>> {
    let q = "INSERT INTO nfe_icms (icms_orig, icms_cst, icms_modbc, icms_vbc, icms_picms, icms_vicms, icms_idproduto) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING icms_idicms";
    match select_icms_id(pool, idproduto).await {
        Ok(imposto_id) => Ok(imposto_id),
        Err(_) => {
            let result = sqlx::query(q)
                .bind(imposto.icms_orig)
                .bind(imposto.icms_cst)
                .bind(imposto.icms_modbc)
                .bind(imposto.icms_vbc)
                .bind(imposto.icms_picms)
                .bind(imposto.icms_vicms)
                .bind(idproduto)
                .fetch_one(pool)
                .await?
                .get::<i32, _>(0);
            Ok(IcmsId {
                icms_idicms: result,
            })
        }
    }
}
