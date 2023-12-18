use crate::modules::json::structs::emit::Emit;
use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::{parse_value_to_i64, parse_value_to_string};
use serde_json::Value;

impl Emit {
    #[allow(dead_code)]
    pub fn empty() -> Emit {
        let value: Emit = Emit {
            cnpj_cpf: 0,
            crt: 0,
            ie: 0,
            iest: 0,
            ender_emit: Ender::empty(),
            x_fant: "Null".to_string(),
            x_nome: "Null".to_string(),
        };
        return value;
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> Emit {
        let base_emit = &base["nfeProc"]["NFe"]["infNFe"]["emit"];
        let value: Emit = Emit {
            cnpj_cpf: parse_value_to_i64(&base_emit["CNPJ"]),
            crt: parse_value_to_i64(&base_emit["CRT"]),
            ie: parse_value_to_i64(&base_emit["IE"]),
            iest: parse_value_to_i64(&base_emit["IEST"]),
            ender_emit: Ender::new_emit(&base),
            x_fant: parse_value_to_string(&base_emit["xFant"]),
            x_nome: parse_value_to_string(&base_emit["xNome"]),
        };
        // println!("{:?}", value);
        return value;
    }
}
