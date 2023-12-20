use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::parse_value_to_string;
use serde_json::Value;

impl Ender {
    #[allow(dead_code)]
    pub fn empty() -> Ender {
        let value: Ender = Ender {
            cep: "Null".to_string(),
            uf: "Null".to_string(),
            c_mun: "Null".to_string(),
            c_pais: "Null".to_string(),
            nro: "Null".to_string(),
            x_bairro: "Null".to_string(),
            x_cpl: "Null".to_string(),
            x_lgr: "Null".to_string(),
            x_mun: "Null".to_string(),
        };
        return value;
    }

    #[allow(dead_code)]
    pub fn new_dest(base: &Value) -> Ender {
        let base_ender_dest = &base["nfeProc"]["NFe"]["infNFe"]["dest"]["enderDest"];
        let value: Ender = Ender {
            cep: parse_value_to_string(&base_ender_dest["CEP"]),
            uf: parse_value_to_string(&base_ender_dest["UF"]),
            c_mun: parse_value_to_string(&base_ender_dest["cMun"]),
            c_pais: parse_value_to_string(&base_ender_dest["cPais"]),
            nro: parse_value_to_string(&base_ender_dest["nro"]),
            x_bairro: parse_value_to_string(&base_ender_dest["xBairro"]),
            x_cpl: parse_value_to_string(&base_ender_dest["xCpl"]),
            x_lgr: parse_value_to_string(&base_ender_dest["xLgr"]),
            x_mun: parse_value_to_string(&base_ender_dest["xMun"]),
        };
        // println!("{:?}", value);
        return value;
    }
    pub fn new_emit(base: &Value) -> Ender {
        let base_ender_emit = &base["nfeProc"]["NFe"]["infNFe"]["emit"]["enderEmit"];
        let value: Ender = Ender {
            cep: parse_value_to_string(&base_ender_emit["CEP"]),
            uf: parse_value_to_string(&base_ender_emit["UF"]),
            c_mun: parse_value_to_string(&base_ender_emit["cMun"]),
            c_pais: parse_value_to_string(&base_ender_emit["cPais"]),
            nro: parse_value_to_string(&base_ender_emit["nro"]),
            x_bairro: parse_value_to_string(&base_ender_emit["xBairro"]),
            x_cpl: parse_value_to_string(&base_ender_emit["xCpl"]),
            x_lgr: parse_value_to_string(&base_ender_emit["xLgr"]),
            x_mun: parse_value_to_string(&base_ender_emit["xMun"]),
        };
        // println!("{:?}", value);
        return value;
    }
}
