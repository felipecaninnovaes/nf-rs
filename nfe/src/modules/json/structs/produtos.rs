use crate::modules::json::structs::impostos::Impostos;

#[derive(Debug, PartialEq)]
pub struct Produto {
    pub n_item: String,
    pub c_prod: String,
    pub c_ean: String,
    pub x_prod: String,
    pub ncm: String,
    pub cfop: String,
    pub u_com: String,
    pub q_com: f64,
    pub v_un_com: f64,
    pub v_prod: f64,
    pub c_eantrib: String,
    pub u_trib: String,
    pub q_trib: f64,
    pub v_un_trib: f64,
    pub ind_tot: String,
    pub x_ped: String,
    pub impostos: Impostos,
}
