#![allow(unused_imports, dead_code, unused_variables)]
use nfe::modules::json::structs::produtos::Produto;
use sqlx::Row;
use std::error::Error;

use crate::modules::{
    nfe::produtos::impostos::{
        cofins::insert::insert_cofins_sql, icms::insert::insert_icms_sql,
        icms_uf_dest::insert::insert_icms_uf_dest_sql, ipi::insert::insert_ipi_sql,
        pis::insert::insert_pis_sql,
    },
    utils::parser::parse_value_to_bigdecimal,
};

use super::select::select_produto_id;

pub async fn insert_produto_sql(
    pool: &sqlx::PgPool,
    produto: &Vec<Produto>,
    idnfe: &i32,
) -> Result<(), i32> {
    for p in produto {
        match select_produto_id(pool, &p.produto_nitem, idnfe).await {
            Ok(idproduto) => {
                println!("Produto já existe");
                insert_cofins_sql(
                    pool,
                    &p.imposto.imposto_cofins,
                    &idproduto.produto_idproduto,
                )
                .await
                .unwrap();
                insert_icms_uf_dest_sql(
                    pool,
                    &p.imposto.imposto_icms_uf_dest,
                    &idproduto.produto_idproduto,
                )
                .await
                .unwrap();
                insert_icms_sql(pool, &p.imposto.imposto_icms, &idproduto.produto_idproduto)
                    .await
                    .unwrap();
                insert_ipi_sql(pool, &p.imposto.imposto_ipi, &idproduto.produto_idproduto)
                    .await
                    .unwrap();
                insert_pis_sql(pool, &p.imposto.imposto_pis, &idproduto.produto_idproduto)
                    .await
                    .unwrap();
            }
            Err(_) => {
                let idproduto = sqlx::query!(
                    r#"INSERT INTO nfe_produto (produto_nitem, produto_cprod, produto_cean, produto_xprod, produto_ncm, produto_cfop, produto_ucom, produto_qcom, produto_vuncom, produto_vprod, produto_ceantrib, produto_utrib, produto_qtrib, produto_vuntrib, produto_indtot, produto_xped, produto_idnfe) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,$15, $16, $17) RETURNING produto_idproduto"#,
                    p.produto_nitem,
                    p.produto_cprod,
                    p.produto_cean,
                    p.produto_xprod,
                    p.produto_ncm,
                    p.produto_cfop,
                    p.produto_ucom,
                    parse_value_to_bigdecimal(&p.produto_qcom),
                    parse_value_to_bigdecimal(&p.produto_vuncom),
                    parse_value_to_bigdecimal(&p.produto_vprod),
                    p.produto_ceantrib,
                    p.produto_utrib,
                    parse_value_to_bigdecimal(&p.produto_qtrib),
                    parse_value_to_bigdecimal(&p.produto_vuntrib),
                    p.produto_indtot,
                    p.produto_xped,
                    idnfe,
                ).fetch_one(pool).await.unwrap().produto_idproduto;

                // insert impostos
                insert_cofins_sql(pool, &p.imposto.imposto_cofins, &idproduto)
                    .await
                    .unwrap();
                insert_icms_uf_dest_sql(pool, &p.imposto.imposto_icms_uf_dest, &idproduto)
                    .await
                    .unwrap();
                insert_icms_sql(pool, &p.imposto.imposto_icms, &idproduto)
                    .await
                    .unwrap();
                insert_ipi_sql(pool, &p.imposto.imposto_ipi, &idproduto)
                    .await
                    .unwrap();
                insert_pis_sql(pool, &p.imposto.imposto_pis, &idproduto)
                    .await
                    .unwrap();
            }
        }
    }
    Ok(())
}
