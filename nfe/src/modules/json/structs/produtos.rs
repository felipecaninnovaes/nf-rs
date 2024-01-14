use crate::modules::json::structs::impostos::Imposto;

#[derive(Debug, PartialEq)]
pub struct Produto {
    pub produto_nitem: String,
    pub produto_cprod: String,
    pub produto_cean: String,
    pub produto_xprod: String,
    pub produto_ncm: String,
    pub produto_cfop: String,
    pub produto_ucom: String,
    pub produto_qcom: f64,
    pub produto_vuncom: f64,
    pub produto_vprod: f64,
    pub produto_ceantrib: String,
    pub produto_utrib: String,
    pub produto_qtrib: f64,
    pub produto_vuntrib: f64,
    pub produto_indtot: String,
    pub produto_xped: String,
    pub imposto: Imposto,
}
