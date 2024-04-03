#![allow(dead_code)]
use nfe::modules::json::structs::{
    emit::{Emit, EmitId},
    ender::EnderId,
};

use crate::modules::nfe::ender::insert::insert_ender_sql;

use super::select::select_emit_id;

pub async fn insert_emit_sql(pool: &sqlx::PgPool, emit: &Emit) -> Result<EmitId, EmitId> {
    match select_emit_id(pool, &emit.emit_cnpjcpf).await {
        Ok(idender) => Ok(idender),
        Err(_) => {
            let emit_idender: EnderId = insert_ender_sql(pool, &emit.ender_emit).await.unwrap();
            let result = sqlx::query!(
                r#"INSERT INTO nfe_emit (emit_cnpjcpf, emit_crt, emit_ie, emit_iest, emit_xfant, emit_xnome, emit_idender ) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING emit_idemit"#,
                emit.emit_cnpjcpf,
                emit.emit_crt,
                emit.emit_ie,
                emit.emit_iest,
                emit.emit_xfant,
                emit.emit_xnome,
                emit_idender.ender_idender,
            ).fetch_one(pool).await.unwrap();
            Ok(EmitId {
                emit_idemit: result.emit_idemit,
            })
        }
    }
}
