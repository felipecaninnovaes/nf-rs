use super::ender::Ender;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Dest {
    pub dest_cnpjcpf: String,
    pub dest_ie: String,
    pub dest_email: String,
    pub dest_ender: Ender,
    pub dest_xnome: String,
    pub dest_indiedest: String,
}
