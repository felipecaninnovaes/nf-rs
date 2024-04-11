use crate::modules::json::structs::emit::Emit;
use crate::modules::json::structs::ender::Ender;
use serde_json::Value;
use utils::core::parser::value_to_string;

impl Emit {
    #[allow(dead_code)]
    pub fn new(base: &Value) -> Emit {
        let base_emit = &base["nfeProc"]["NFe"]["infNFe"]["emit"];
        Emit {
            emit_cnpjcpf: value_to_string(&base_emit["CNPJ"]),
            emit_crt: value_to_string(&base_emit["CRT"]),
            emit_ie: value_to_string(&base_emit["IE"]),
            emit_iest: value_to_string(&base_emit["IEST"]),
            emit_xfant: value_to_string(&base_emit["xFant"]),
            emit_xnome: value_to_string(&base_emit["xNome"]),
            ender_emit: Ender::new_emit(base),
        }
    }

    pub fn empty() -> Emit {
        Emit {
            emit_cnpjcpf: "Null".to_string(),
            emit_crt: "Null".to_string(),
            emit_ie: "Null".to_string(),
            emit_iest: "Null".to_string(),
            emit_xfant: "Null".to_string(),
            emit_xnome: "Null".to_string(),
            ender_emit: Ender::empty(),
        }
    }
}
