use crate::modules::json::structs::impostos::Icms;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;

impl Icms {
    #[allow(dead_code)]
    pub fn empty() -> Icms {
        Icms {
            icms_orig: 0,
            icms_cst: 0,
            icms_modbc: 0,
            icms_vbc: 0.0,
            icms_picms: 0.0,
            icms_vicms: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> Icms {
        let prod_icms = &base["imposto"]["ICMS"]["ICMS00"];
        Icms {
            icms_orig: parse_value_to_i64(&prod_icms["orig"]),
            icms_cst: parse_value_to_i64(&prod_icms["CST"]),
            icms_modbc: parse_value_to_i64(&prod_icms["modBC"]),
            icms_vbc: parse_value_to_f64(&prod_icms["vBC"]),
            icms_picms: parse_value_to_f64(&prod_icms["pICMS"]),
            icms_vicms: parse_value_to_f64(&prod_icms["vICMS"]),
        }
    }
}
