#![allow(unused_imports)]
use nfe::modules::json::structs::emit::Emit;
use sqlx::Row;
use std::error::Error;

use crate::modules::nfe::ender::insert::insert_ender_sql;

use super::select::select_emit_id;
#[allow(dead_code)]
pub async fn insert_emit(pool: &sqlx::PgPool, emit: &Emit) -> Result<i32, i32> {
    match select_emit_id(pool, &emit.emit_cnpjcpf).await {
        Ok(idemit) => Ok(idemit.emit_idemit),
        Err(_) => {
            let idender = insert_ender_sql(pool, &emit.ender_emit).await.unwrap();
            let q = "INSERT INTO nfe_emit (emit_cnpjcpf, emit_crt, emit_ie, emit_iest, emit_xfant, emit_xnome, emit_idender ) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING emit_idemit";
            let idemit = sqlx::query(q)
                .bind(&emit.emit_cnpjcpf)
                .bind(&emit.emit_crt)
                .bind(&emit.emit_ie)
                .bind(&emit.emit_iest)
                .bind(&emit.emit_xfant)
                .bind(&emit.emit_xnome)
                .bind(idender.ender_idender)
                .fetch_one(pool)
                .await
                .unwrap()
                .get::<i32, _>(0);
            Ok(idemit)
        }
    }
}
// Ok(Result)
