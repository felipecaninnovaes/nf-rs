use crate::modules::json::structs::dest::Dest;
use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::{parse_value_to_i64, parse_value_to_string};
use serde_json::Value;

impl Dest {
    #[allow(dead_code)]
    pub fn empty() -> Dest {
        let value: Dest = Dest {
            cnpj_cpf: 0,
            ie: 0,
            email: "Null".to_string(),
            ender_dest: Ender::empty(),
            ind_iedest: 0,
            x_nome: "Null".to_string(),
        };
        return value;
    }

    #[allow(dead_code)]
    pub fn new(dest_cnpj: &Value, base: &Value) -> Dest {
        let base_dest = &base["nfeProc"]["NFe"]["infNFe"]["dest"];
        if dest_cnpj != &Value::Null {
            let result: Dest = Dest {
                cnpj_cpf: parse_value_to_i64(&base_dest["CNPJ"]),
                ie: parse_value_to_i64(&base_dest["IE"]),
                email: parse_value_to_string(&base_dest["email"]),
                ender_dest: Ender::new_dest(&base),
                ind_iedest: parse_value_to_i64(&base_dest["indIEDest"]),
                x_nome: parse_value_to_string(&base_dest["xNome"]),
            };
            // println!("{:?}", result);
            return result;
        } else {
            let result: Dest = Dest {
                cnpj_cpf: parse_value_to_i64(&base_dest["CPF"]),
                ie: parse_value_to_i64(&base_dest["IE"]),
                email: parse_value_to_string(&base_dest["email"]),
                ender_dest: Ender::new_dest(&base),
                ind_iedest: parse_value_to_i64(&base_dest["indIEDest"]),
                x_nome: parse_value_to_string(&base_dest["xNome"]),
            };
            // println!("{:?}", result);
            return result;
        }
    }
}
