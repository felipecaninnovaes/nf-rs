#![allow(dead_code)]
use nfe::modules::json::structs::{
    dest::{Dest, DestId},
    ender::EnderId,
};

use crate::modules::nfe::ender::insert::insert_ender_sql;

use super::select::select_dest_id;

pub async fn insert_dest_sql(pool: &sqlx::PgPool, dest: &Dest) -> Result<DestId, DestId> {
    match select_dest_id(pool, &dest.dest_cnpjcpf).await {
        Ok(idender) => Ok(idender),
        Err(_) => {
            let dest_idender: EnderId = insert_ender_sql(pool, &dest.dest_ender).await.unwrap();
            let result = sqlx::query!(
                r#"INSERT INTO nfe_dest (dest_cnpjcpf, dest_ie, dest_email, dest_indiedest, dest_xnome, dest_idender) VALUES ($1, $2, $3, $4, $5, $6) RETURNING dest_iddest"#,
                dest.dest_cnpjcpf,
                dest.dest_ie,
                dest.dest_email,
                dest.dest_indiedest,
                dest.dest_xnome,
                dest_idender.ender_idender,
            ).fetch_one(pool).await.unwrap();
            Ok(DestId {
                dest_iddest: result.dest_iddest,
            })
        }
    }
}
