use super::ender::Ender;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Dest {
    pub cnpj_cpf: i64,
    pub ie: i64,
    pub email: String,
    pub ender_dest: Ender,
    pub ind_iedest: i64,
    pub x_nome: String,
}
