use crate::modules::json::structs::dest::Dest;
use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::parse_value_to_string;
use serde_json::Value;

impl Dest {
    #[allow(dead_code)]
    pub fn empty() -> Dest {
        Dest {
            dest_cnpjcpf: "Null".to_string(),
            dest_ie: "Null".to_string(),
            dest_email: "Null".to_string(),
            dest_ender: Ender::empty(),
            dest_indiedest: "Null".to_string(),
            dest_xnome: "Null".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> Dest {
        let base_dest = &base["nfeProc"]["NFe"]["infNFe"]["dest"];
        let dest_cnpj = &base_dest["CNPJ"];
        let cnpj_cpf = if dest_cnpj != &Value::Null {
            parse_value_to_string(dest_cnpj)
        } else {
            parse_value_to_string(&base_dest["CPF"])
        };
        
        let result: Dest = Dest {
            dest_cnpjcpf: cnpj_cpf,
            dest_ie: parse_value_to_string(&base_dest["IE"]),
            dest_email: parse_value_to_string(&base_dest["email"]),
            dest_ender: Ender::new_dest(base),
            dest_indiedest: parse_value_to_string(&base_dest["indIEDest"]),
            dest_xnome: parse_value_to_string(&base_dest["xNome"]),
        };
        result
    }
}