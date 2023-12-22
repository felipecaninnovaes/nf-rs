use crate::modules::json::structs::impostos::Icms;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;

impl Icms {
    #[allow(dead_code)]
    pub fn empty() -> Icms {
        Icms {
            orig: 0,
            cst: 0,
            mod_bc: 0,
            v_bc: 0.0,
            p_icms: 0.0,
            v_icms: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> Icms {
        let prod_icms = &base["imposto"]["ICMS"]["ICMS00"];
        Icms {
            orig: parse_value_to_i64(&prod_icms["orig"]),
            cst: parse_value_to_i64(&prod_icms["CST"]),
            mod_bc: parse_value_to_i64(&prod_icms["modBC"]),
            v_bc: parse_value_to_f64(&prod_icms["vBC"]),
            p_icms: parse_value_to_f64(&prod_icms["pICMS"]),
            v_icms: parse_value_to_f64(&prod_icms["vICMS"]),
        }
    }
}
