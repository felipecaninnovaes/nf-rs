use crate::modules::json::structs::ender::Ender;
use crate::modules::util::parse_utils::parse_value_to_string;
use serde_json::Value;

impl Ender {
    #[allow(dead_code)]
    pub fn empty() -> Ender {
        Ender {
            ender_cep: "Null".to_string(),
            ender_uf: "Null".to_string(),
            ender_cmun: "Null".to_string(),
            ender_cpais: "Null".to_string(),
            ender_nro: "Null".to_string(),
            ender_xbairro: "Null".to_string(),
            ender_xcpl: "Null".to_string(),
            ender_xlgr: "Null".to_string(),
            ender_xmun: "Null".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn new_dest(base: &Value) -> Ender {
        let base_ender_dest = &base["nfeProc"]["NFe"]["infNFe"]["dest"]["enderDest"];
        Ender {
            ender_cep: parse_value_to_string(&base_ender_dest["CEP"]),
            ender_uf: parse_value_to_string(&base_ender_dest["UF"]),
            ender_cmun: parse_value_to_string(&base_ender_dest["cMun"]),
            ender_cpais: parse_value_to_string(&base_ender_dest["cPais"]),
            ender_nro: parse_value_to_string(&base_ender_dest["nro"]),
            ender_xbairro: parse_value_to_string(&base_ender_dest["xBairro"]),
            ender_xcpl: parse_value_to_string(&base_ender_dest["xCpl"]),
            ender_xlgr: parse_value_to_string(&base_ender_dest["xLgr"]),
            ender_xmun: parse_value_to_string(&base_ender_dest["xMun"]),
        }
    }

    pub fn new_emit(base: &Value) -> Ender {
        let base_ender_emit = &base["nfeProc"]["NFe"]["infNFe"]["emit"]["enderEmit"];
        Ender {
            ender_cep: parse_value_to_string(&base_ender_emit["CEP"]),
            ender_uf: parse_value_to_string(&base_ender_emit["UF"]),
            ender_cmun: parse_value_to_string(&base_ender_emit["cMun"]),
            ender_cpais: parse_value_to_string(&base_ender_emit["cPais"]),
            ender_nro: parse_value_to_string(&base_ender_emit["nro"]),
            ender_xbairro: parse_value_to_string(&base_ender_emit["xBairro"]),
            ender_xcpl: parse_value_to_string(&base_ender_emit["xCpl"]),
            ender_xlgr: parse_value_to_string(&base_ender_emit["xLgr"]),
            ender_xmun: parse_value_to_string(&base_ender_emit["xMun"]),
        }
    }
}
