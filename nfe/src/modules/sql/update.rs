use std::error::Error;

use crate::modules::json::structs::nfe_struct::NfeUpdate;

pub async fn update_nfe(pool: &sqlx::PgPool, nfe: NfeUpdate) -> Result<(), Box<dyn Error>> {
    let q = format!("UPDATE nfe SET nfe_cdv = '{}', nfe_cmunfg = '{}', nfe_cnf = '{}', nfe_cuf = '{}', nfe_dhemi = '{}', nfe_dhsaient = '{}', nfe_finnfe = '{}', nfe_nfe_iddest = '{}', nfe_indfinal = '{}', nfe_indintermed = '{}', nfe_indpres = '{}', nfe_modnfe = '{}', nfe_nnf = '{}', nfe_natop = '{}', nfe_procemi = '{}', nfe_serie = '{}', nfe_tpamb = '{}', nfe_tpemis = '{}', nfe_tpimp = '{}', nfe_tpnf = '{}', nfe_verproc = '{}', nfe_nftotal = '{}', nfe_idemit = '{}', nfe_iddest = '{}' WHERE nfe_idnfe = '{}'", nfe.nfe_cdv, nfe.nfe_cmunfg, nfe.nfe_cnf, nfe.nfe_cuf, nfe.nfe_dhemi, nfe.nfe_dhsaient, nfe.nfe_finnfe, nfe.nfe_nfe_iddest, nfe.nfe_indfinal, nfe.nfe_indintermed, nfe.nfe_indpres, nfe.nfe_modnfe, nfe.nfe_nnf, nfe.nfe_natop, nfe.nfe_procemi, nfe.nfe_serie, nfe.nfe_tpamb, nfe.nfe_tpemis, nfe.nfe_tpimp, nfe.nfe_tpnf, nfe.nfe_verproc, nfe.nfe_nftotal, nfe.nfe_idemit, nfe.nfe_iddest, nfe.nfe_idnfe);
    sqlx::query(&q).execute(pool).await?;
    Ok(())
}
