#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::{Icms, IcmsId};
use sqlx::Row;
use std::error::Error;

use crate::modules::utils::parser::parse_value_to_bigdecimal;

use super::select::select_icms_id;

pub async fn insert_icms_sql(
    pool: &sqlx::PgPool,
    imposto: &Icms,
    idproduto: &i32,
) -> Result<IcmsId, Box<dyn Error>> {
    match select_icms_id(pool, idproduto).await {
        Ok(imposto_id) => Ok(imposto_id),
        Err(_) => {
            let map = sqlx::query!(
                r#"INSERT INTO nfe_icms (icms_orig, icms_cst, icms_modbc, icms_vbc, icms_picms, icms_vicms, icms_idproduto) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id"#,
                imposto.icms_orig.to_string(),
                imposto.icms_cst.to_string(),
                imposto.icms_modbc.to_string(),
                parse_value_to_bigdecimal(&imposto.icms_vbc),
                parse_value_to_bigdecimal(&imposto.icms_picms),
                parse_value_to_bigdecimal(&imposto.icms_vicms),
                idproduto,
            );
            let result = map.fetch_one(pool).await.unwrap();
            Ok(IcmsId {
                icms_idicms: result.id,
            })
        }
    }
}
