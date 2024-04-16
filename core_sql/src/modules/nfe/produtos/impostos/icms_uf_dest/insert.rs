#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::impostos::{IcmsUfDest, IcmsUfDestId};
use sqlx::Row;
use std::error::Error;

use crate::modules::utils::parser::parse_value_to_bigdecimal;

use super::select::select_icms_uf_dest_id;

pub async fn insert_icms_uf_dest_sql(
    pool: &sqlx::PgPool,
    imposto: &IcmsUfDest,
    idproduto: &i32,
) -> Result<IcmsUfDestId, Box<dyn Error>> {
    match select_icms_uf_dest_id(pool, idproduto).await {
        Ok(imposto_id) => Ok(imposto_id),
        Err(_) => {
            let map = sqlx::query!(
                r#"INSERT INTO nfe_icmsufdest (icms_uf_vbcufdest, icms_uf_vbcfcpufdest, icms_uf_pfcpufdest, icms_uf_picmsufdest, icms_uf_picmsinter, icms_uf_picmsinterpart, icms_uf_vfcpufdest, icms_uf_vicmsufdest, icms_uf_vicmsufremet, icms_uf_idproduto) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id"#,
                parse_value_to_bigdecimal(&imposto.icms_uf_vbcufdest),
                parse_value_to_bigdecimal(&imposto.icms_uf_vbcfcpufdest),
                parse_value_to_bigdecimal(&imposto.icms_uf_pfcpufdest),
                parse_value_to_bigdecimal(&imposto.icms_uf_picmsufdest),
                parse_value_to_bigdecimal(&imposto.icms_uf_picmsinter),
                parse_value_to_bigdecimal(&imposto.icms_uf_picmsinterpart),
                parse_value_to_bigdecimal(&imposto.icms_uf_vfcpufdest),
                parse_value_to_bigdecimal(&imposto.icms_uf_vicmsufdest),
                parse_value_to_bigdecimal(&imposto.icms_uf_vicmsufremet),
                idproduto,
            );
            let result = map.fetch_one(pool).await.unwrap();
            Ok(IcmsUfDestId {
                icms_uf_idicmsufdest: result.id,
            })
        }
    }
}
