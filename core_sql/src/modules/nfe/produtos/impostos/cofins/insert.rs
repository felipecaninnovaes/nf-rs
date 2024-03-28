#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::{
    json::structs::impostos::{Cofins, CofinsId},
    sql::select,
};
use sqlx::Row;
use std::error::Error;

use crate::modules::utils::parser::parse_value_to_bigdecimal;

use super::select::select_cofins_id;

pub async fn insert_cofins_sql(
    pool: &sqlx::PgPool,
    imposto: &Cofins,
    idproduto: &i32,
) -> Result<CofinsId, Box<dyn Error>> {
    // let q = "INSERT INTO nfe_cofins (cofins_cst, cofins_vbc, cofins_pcofins, cofins_vcofins, cofins_idproduto) VALUES ($1, $2, $3, $4, $5) RETURNING cofins_idcofins";
    match select_cofins_id(pool, idproduto).await {
        Ok(imposto_id) => Ok(imposto_id),
        Err(_) => {
            let map = sqlx::query!(
                r#"INSERT INTO nfe_cofins (cofins_cst, cofins_vbc, cofins_pcofins, cofins_vcofins, cofins_idproduto) VALUES ($1, $2, $3, $4, $5) RETURNING cofins_idcofins"#,
                imposto.cofins_cst.to_string(),
                parse_value_to_bigdecimal(&imposto.cofins_vbc),
                parse_value_to_bigdecimal(&imposto.cofins_pcofins),
                parse_value_to_bigdecimal(&imposto.cofins_vcofins),
                idproduto,
            );
            let result = map.fetch_one(pool).await.unwrap();
            // let result = sqlx::query(q)
            //     .bind(imposto.cofins_cst)
            //     .bind(imposto.cofins_vbc)
            //     .bind(imposto.cofins_pcofins)
            //     .bind(imposto.cofins_vcofins)
            //     .bind(idproduto)
            //     .fetch_one(pool)
            //     .await?
            //     .get::<i32, _>(0);
            Ok(CofinsId {
                cofins_idcofins: result.cofins_idcofins,
            })
        }
    }
}
