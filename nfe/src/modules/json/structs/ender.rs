use sqlx::FromRow;

#[derive(Debug, PartialEq)]
pub struct Ender {
    pub ender_cep: String,
    pub ender_uf: String,
    pub ender_cmun: String,
    pub ender_cpais: String,
    pub ender_nro: String,
    pub ender_xbairro: String,
    pub ender_xcpl: String,
    pub ender_xlgr: String,
    pub ender_xmun: String,
}

#[derive(Debug, PartialEq, FromRow)]
pub struct EnderId {
  pub ender_idender: i32,
}
