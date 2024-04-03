use nfe::modules::json::structs::ender::{Ender, EnderId};

use super::select::select_ender_id;

pub async fn insert_ender_sql(pool: &sqlx::PgPool, ender: &Ender) -> Result<EnderId, EnderId> {
    match select_ender_id(pool, &ender.ender_nro, &ender.ender_cep).await {
        Ok(idender) => Ok(idender),
        Err(_) => {
            let result = sqlx::query!(
                r#"INSERT INTO nfe_ender (ender_cep, ender_uf, ender_cmun, ender_cpais, ender_nro, ender_xbairro, ender_xcpl, ender_xlgr, ender_xmun) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING ender_idender"#,
                ender.ender_cep,
                ender.ender_uf,
                ender.ender_cmun,
                ender.ender_cpais,
                ender.ender_nro,
                ender.ender_xbairro,
                ender.ender_xcpl,
                ender.ender_xlgr,
                ender.ender_xmun
            ).fetch_one(pool).await.unwrap();
            Ok(EnderId {
                ender_idender: result.ender_idender,
            })
        }
    }
}
