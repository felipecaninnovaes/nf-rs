use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::{parse_value_to_string, parse_value_to_i64};
use serde_json::Value;

impl Ender {
    pub fn empty() -> Ender {
        let value: Ender = Ender {
            cep: 0,
            uf: "Null".to_string(),
            c_mun: 0,
            c_pais: 0,
            nro: 0,
            x_bairro: "Null".to_string(),
            x_cpl: "Null".to_string(),
            x_lgr: "Null".to_string(),
            x_mun: "Null".to_string(),
        };
        return value;
    }

    pub fn new(base: &Value) -> Ender {
        let base_ender = &base["nfeProc"]["NFe"]["infNFe"]["dest"]["enderDest"];
        let value: Ender = Ender { cep: parse_value_to_i64(&base_ender["CEP"]), uf: parse_value_to_string(&base_ender["UF"]), c_mun: parse_value_to_i64(&base_ender["cMun"]), c_pais: parse_value_to_i64(&base_ender["cPais"]), nro: parse_value_to_i64(&base_ender["nro"]), x_bairro: parse_value_to_string(&base_ender["xBairro"]), x_cpl: parse_value_to_string(&base_ender["xCpl"]), x_lgr: parse_value_to_string(&base_ender["xLgr"]), x_mun: parse_value_to_string(&base_ender["xMun"]) };
        return value;
    }
}
