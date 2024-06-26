use super::{dest::Dest, emit::Emit, produtos::Produto};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NfeId {
    pub nfe_idnfe: i32,
}

#[derive(Debug, PartialEq)]
pub struct Nfe {
    pub nfe_cdv: String,
    pub nfe_cmunfg: String,
    pub nfe_cnf: String,
    pub nfe_cuf: String,
    pub nfe_dhemi: String,
    pub nfe_dhsaient: String,
    pub nfe_finnfe: String,
    pub nfe_nfe_iddest: String,
    pub nfe_indfinal: String,
    pub nfe_indintermed: String,
    pub nfe_indpres: String,
    pub nfe_modnfe: String,
    pub nfe_nnf: String,
    pub nfe_natop: String,
    pub nfe_procemi: String,
    pub nfe_serie: String,
    pub nfe_tpamb: String,
    pub nfe_tpemis: String,
    pub nfe_tpimp: String,
    pub nfe_tpnf: String,
    pub nfe_verproc: String,
    pub nfe_nftotal: f64,
    pub nfe_emit: Emit,
    pub nfe_dest: Dest,
    pub nfe_produtos: Vec<Produto>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NfeSelect {
    pub nfe_idnfe: i32,
    pub nfe_cdv: String,
    pub nfe_cmunfg: String,
    pub nfe_cnf: String,
    pub nfe_cuf: String,
    pub nfe_dhemi: String,
    pub nfe_dhsaient: String,
    pub nfe_finnfe: String,
    pub nfe_nfe_iddest: String,
    pub nfe_indfinal: String,
    pub nfe_indintermed: String,
    pub nfe_indpres: String,
    pub nfe_modnfe: String,
    pub nfe_nnf: String,
    pub nfe_natop: String,
    pub nfe_procemi: String,
    pub nfe_serie: String,
    pub nfe_tpamb: String,
    pub nfe_tpemis: String,
    pub nfe_tpimp: String,
    pub nfe_tpnf: String,
    pub nfe_verproc: String,
    pub nfe_nftotal: String,
    pub nfe_idemit: i32,
    pub nfe_iddest: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NfeJoinSelect {
    pub nfe_idnfe: i32,
    pub nfe_cdv: String,
    pub nfe_cmunfg: String,
    pub nfe_cnf: String,
    pub nfe_cuf: String,
    pub nfe_dhemi: String,
    pub nfe_dhsaient: String,
    pub nfe_finnfe: String,
    pub nfe_nfe_iddest: String,
    pub nfe_indfinal: String,
    pub nfe_indintermed: String,
    pub nfe_indpres: String,
    pub nfe_modnfe: String,
    pub nfe_nnf: String,
    pub nfe_natop: String,
    pub nfe_procemi: String,
    pub nfe_serie: String,
    pub nfe_tpamb: String,
    pub nfe_tpemis: String,
    pub nfe_tpimp: String,
    pub nfe_tpnf: String,
    pub nfe_verproc: String,
    pub nfe_nftotal: String,
    pub nfe_idemit: i32,
    pub nfe_iddest: i32,
    pub emit_idemit: i32,
    pub emit_cnpjcpf: String,
    pub emit_crt: String,
    pub emit_ie: String,
    pub emit_iest: String,
    pub emit_xfant: String,
    pub emit_xnome: String,
    pub emit_idender: i32,
    pub dest_iddest: i32,
    pub dest_cnpjcpf: String,
    pub dest_ie: String,
    pub dest_email: String,
    pub dest_indiedest: String,
    pub dest_xnome: String,
    pub dest_idender: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NfeUpdate {
    pub nfe_idnfe: i32,
    pub nfe_cdv: String,
    pub nfe_cmunfg: String,
    pub nfe_cnf: String,
    pub nfe_cuf: String,
    pub nfe_dhemi: String,
    pub nfe_dhsaient: String,
    pub nfe_finnfe: String,
    pub nfe_nfe_iddest: String,
    pub nfe_indfinal: String,
    pub nfe_indintermed: String,
    pub nfe_indpres: String,
    pub nfe_modnfe: String,
    pub nfe_nnf: String,
    pub nfe_natop: String,
    pub nfe_procemi: String,
    pub nfe_serie: String,
    pub nfe_tpamb: String,
    pub nfe_tpemis: String,
    pub nfe_tpimp: String,
    pub nfe_tpnf: String,
    pub nfe_verproc: String,
    pub nfe_nftotal: String,
    pub nfe_idemit: i32,
    pub nfe_iddest: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permissions {
    pub permissions_user_id: Uuid,
    pub permissions_allowed: bool,
    pub cnpj: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PermissionsUpdate {
    pub permissions_idpermission: Uuid,
    pub permissions_user_id: Uuid,
    pub permissions_allowed: bool,
    pub cnpj: String,
}
