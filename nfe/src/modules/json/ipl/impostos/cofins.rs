use crate::modules::json::structs::impostos::Cofins;
use serde_json::Value;
use utils::core::parser::{value_to_f64, value_to_i64};
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
            cofins_cst: value_to_i64(&prod_cofins["CST"]),
            cofins_vbc: value_to_f64(&prod_cofins["vBC"]),
            cofins_pcofins: value_to_f64(&prod_cofins["pCOFINS"]),
            cofins_vcofins: value_to_f64(&prod_cofins["vCOFINS"]),
        }
    }
}
