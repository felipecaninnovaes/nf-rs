#![allow(unused_imports, dead_code, unused_variables)]
use bigdecimal::ToPrimitive;
use nfe::modules::json::structs::impostos::{Ipi, IpiId};
use sqlx::{types::BigDecimal, Row};
use std::{error::Error, str::FromStr};

use crate::modules::utils::parser::parse_value_to_bigdecimal;

use super::select::select_ipi_id;

pub async fn insert_ipi_sql(
    pool: &sqlx::PgPool,
    imposto: &Ipi,
    idproduto: &i32,
) -> Result<IpiId, Box<dyn Error>> {
    match select_ipi_id(pool, idproduto).await {
        Ok(imposto_id) => Ok(imposto_id),
        Err(_) => {
            let ipi_vbc_bigdecimal = BigDecimal::from_str(&imposto.ipi_vbc.to_string()).unwrap();
            let map = sqlx::query!(
                r#"INSERT INTO nfe_ipi (ipi_cenq, ipi_cst, ipi_vbc, ipi_pipi, ipi_vipi, ipi_idproduto) VALUES ($1, $2, $3, $4, $5, $6) RETURNING ipi_idipi"#,
                imposto.ipi_cenq.to_string(),
                imposto.ipi_cst.to_string(),
                parse_value_to_bigdecimal(&imposto.ipi_vbc),
                imposto.ipi_pipi.to_string(),
                imposto.ipi_vipi.to_string(),
                idproduto,
            );
            let result = map.fetch_one(pool).await.unwrap();
            Ok(IpiId {
                ipi_idipi: result.ipi_idipi,
            })
        }
    }
}
