use crate::modules::json::structs::dest::Dest;
use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::parse_value_to_string;
use serde_json::Value;

impl Dest {
    #[allow(dead_code)]
    pub fn empty() -> Dest {
        Dest {
            cnpj_cpf: "Null".to_string(),
            ie: "Null".to_string(),
            email: "Null".to_string(),
            ender_dest: Ender::empty(),
            ind_iedest: "Null".to_string(),
            x_nome: "Null".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> Dest {
        let base_dest = &base["nfeProc"]["NFe"]["infNFe"]["dest"];
        let dest_cnpj = &base_dest["CNPJ"];
        let cnpj_cpf = if dest_cnpj != &Value::Null {
            parse_value_to_string(&dest_cnpj)
        } else {
            parse_value_to_string(&base_dest["CPF"])
        };
        
        let result: Dest = Dest {
            cnpj_cpf,
            ie: parse_value_to_string(&base_dest["IE"]),
            email: parse_value_to_string(&base_dest["email"]),
            ender_dest: Ender::new_dest(&base),
            ind_iedest: parse_value_to_string(&base_dest["indIEDest"]),
            x_nome: parse_value_to_string(&base_dest["xNome"]),
        };
        result
    }
}