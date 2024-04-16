#![allow(dead_code)]
use nfe::modules::json::structs::{
    dest::DestId,
    emit::EmitId,
    nfe_struct::{Nfe, NfeId},
};

use crate::modules::nfe::{dest::insert::insert_dest_sql, emit::insert::insert_emit_sql};

use super::select::select_nfe_id;

pub async fn insert_nfe_sql(pool: &sqlx::PgPool, nfe: &Nfe) -> Result<NfeId, NfeId> {
    let iddest: DestId = insert_dest_sql(pool, &nfe.nfe_dest).await.unwrap();
    let idemit: EmitId = insert_emit_sql(pool, &nfe.nfe_emit).await.unwrap();

    match select_nfe_id(pool, &nfe.nfe_nnf, &idemit.emit_idemit).await {
        Ok(idnfe) => Ok(idnfe),
        Err(_) => {
            let result = sqlx::query!(
                r#"INSERT INTO nfe (nfe_cdv, nfe_cmunfg, nfe_cnf, nfe_cuf, nfe_dhemi, nfe_dhsaient, nfe_finnfe, nfe_nfe_iddest, nfe_indfinal, nfe_indintermed, nfe_indpres, nfe_modnfe, nfe_nnf, nfe_natop, nfe_procemi, nfe_serie, nfe_tpamb, nfe_tpemis, nfe_tpimp, nfe_tpnf, nfe_verproc, nfe_nftotal, nfe_idemit, nfe_iddest ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24) RETURNING id"#,
                nfe.nfe_cdv,
                nfe.nfe_cmunfg,
                nfe.nfe_cnf,
                nfe.nfe_cuf,
                nfe.nfe_dhemi,
                nfe.nfe_dhsaient,
                nfe.nfe_finnfe,
                nfe.nfe_nfe_iddest,
                nfe.nfe_indfinal,
                nfe.nfe_indintermed,
                nfe.nfe_indpres,
                nfe.nfe_modnfe,
                nfe.nfe_nnf,
                nfe.nfe_natop,
                nfe.nfe_procemi,
                nfe.nfe_serie,
                nfe.nfe_tpamb,
                nfe.nfe_tpemis,
                nfe.nfe_tpimp,
                nfe.nfe_tpnf,
                nfe.nfe_verproc,
                nfe.nfe_nftotal.to_string(),
                idemit.emit_idemit,
                iddest.dest_iddest,
            ).fetch_one(pool).await.unwrap();
            Ok(NfeId {
                nfe_idnfe: result.id,
            })
        }
    }
}

// Ok(Result)
