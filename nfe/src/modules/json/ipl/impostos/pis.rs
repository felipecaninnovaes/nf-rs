use crate::modules::json::structs::impostos::Pis;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;

impl Pis {
    #[allow(dead_code)]
    pub fn empty() -> Pis {
        let value: Pis = Pis {
            cst: 0,
            v_bc: 0.0,
            p_pis: 0.0,
            v_pis: 0.0,
        };
        return value;
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> Pis {
        let prod_pis = &base["imposto"]["PIS"]["PISAliq"];
        let value: Pis = Pis {
            cst: parse_value_to_i64(&prod_pis["CST"]),
            v_bc: parse_value_to_f64(&prod_pis["vBC"]),
            p_pis: parse_value_to_f64(&prod_pis["pPIS"]),
            v_pis: parse_value_to_f64(&prod_pis["vPIS"]),
        };
        return value;
    }
}
