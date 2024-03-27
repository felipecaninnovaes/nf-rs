use sqlx::prelude::FromRow;

use super::impostos::Imposto;

#[derive(Debug, PartialEq, FromRow)]
pub struct ProdutoId {
    pub produto_idproduto: i32,
}

#[derive(Debug, PartialEq)]
pub struct Produto {
    pub produto_nitem: String,
    pub produto_cprod: String,
    pub produto_cean: String,
    pub produto_xprod: String,
    pub produto_ncm: String,
    pub produto_cfop: String,
    pub produto_ucom: String,
    pub produto_qcom: String,
    pub produto_vuncom: String,
    pub produto_vprod: String,
    pub produto_ceantrib: String,
    pub produto_utrib: String,
    pub produto_qtrib: String,
    pub produto_vuntrib: String,
    pub produto_indtot: String,
    pub produto_xped: String,
    pub imposto: Imposto,
}
