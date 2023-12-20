use super::ender::Ender;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Dest {
    pub cnpj_cpf: String,
    pub ie: String,
    pub email: String,
    pub ender_dest: Ender,
    pub ind_iedest: String,
    pub x_nome: String,
}
