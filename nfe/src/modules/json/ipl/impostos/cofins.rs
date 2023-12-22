use crate::modules::json::structs::impostos::Cofins;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;
impl Cofins {
    #[allow(dead_code)]
    pub fn empty() -> Cofins {
        Cofins {
            cst: 0,
            v_bc: 0.0,
            p_cofins: 0.0,
            v_cofins: 0.0,
        }
    }
    #[allow(dead_code)]
    pub fn new(base: &Value) -> Cofins {
        let prod_cofins = &base["imposto"]["COFINS"]["COFINSAliq"];
        Cofins {
            cst: parse_value_to_i64(&prod_cofins["CST"]),
            v_bc: parse_value_to_f64(&prod_cofins["vBC"]),
            p_cofins: parse_value_to_f64(&prod_cofins["pCOFINS"]),
            v_cofins: parse_value_to_f64(&prod_cofins["vCOFINS"]),
        }
    }
}
