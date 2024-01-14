use crate::modules::json::structs::impostos::Cofins;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;
impl Cofins {
    #[allow(dead_code)]
    pub fn empty() -> Cofins {
        Cofins {
            cofins_cst: 0,
            cofins_vbc: 0.0,
            cofins_pcofins: 0.0,
            cofins_vcofins: 0.0,
        }
    }
    #[allow(dead_code)]
    pub fn new(base: &Value) -> Cofins {
        let prod_cofins = &base["imposto"]["COFINS"]["COFINSAliq"];
        Cofins {
            cofins_cst: parse_value_to_i64(&prod_cofins["CST"]),
            cofins_vbc: parse_value_to_f64(&prod_cofins["vBC"]),
            cofins_pcofins: parse_value_to_f64(&prod_cofins["pCOFINS"]),
            cofins_vcofins: parse_value_to_f64(&prod_cofins["vCOFINS"]),
        }
    }
}
