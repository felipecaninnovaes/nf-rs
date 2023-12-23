use super::ender::Ender;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Emit {
    pub cnpj_cpf: String,
    pub crt: String,
    pub ie: String,
    pub iest: String,
    pub ender_emit: Ender,
    pub x_fant: String,
    pub x_nome: String,
}
