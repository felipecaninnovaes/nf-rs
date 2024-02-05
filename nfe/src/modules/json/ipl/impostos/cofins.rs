use crate::modules::json::structs::impostos::Cofins;
use crate::modules::util::parse_utils::{parse_value_to_i64, parse_value_to_string};
use serde_json::Value;
impl Cofins {
    #[allow(dead_code)]
    pub fn empty() -> Cofins {
        Cofins {
            cofins_cst: 0,
            cofins_vbc: "0.0".to_string(),
            cofins_pcofins: "0.0".to_string(),
            cofins_vcofins: "0.0".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn new(base: &Value) -> Cofins {
        let prod_cofins = &base["imposto"]["COFINS"]["COFINSAliq"];
        Cofins {
            cofins_cst: parse_value_to_i64(&prod_cofins["CST"]),
            cofins_vbc: parse_value_to_string(&prod_cofins["vBC"]),
            cofins_pcofins: parse_value_to_string(&prod_cofins["pCOFINS"]),
            cofins_vcofins: parse_value_to_string(&prod_cofins["vCOFINS"]),
        }
    }
}
