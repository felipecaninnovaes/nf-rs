use crate::modules::json::structs::emit::Emit;
use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;

impl Emit {
    pub fn empty() -> Emit{
        let  value: Emit = Emit { cnpj_cpf: 0, crt: 0, ie: 0, iest: 0, ender_emit: Ender::empty(), x_fant: "Null".to_string(), x_nome: "Null".to_string() };
        return value;
    }
}