use super::ender::Ender;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Emit {
    pub emit_cnpjcpf: String,
    pub emit_crt: String,
    pub emit_ie: String,
    pub emit_iest: String,
    pub ender_emit: Ender,
    pub emit_xfant: String,
    pub emit_xnome: String,
}
