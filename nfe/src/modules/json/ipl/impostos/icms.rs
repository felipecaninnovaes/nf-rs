use crate::modules::json::structs::impostos::Icms;
use serde_json::Value;
use utils::core::parser::{value_to_f64, value_to_i64};

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
            icms_orig: value_to_i64(&prod_icms["orig"]),
            icms_cst: value_to_i64(&prod_icms["CST"]),
            icms_modbc: value_to_i64(&prod_icms["modBC"]),
            icms_vbc: value_to_f64(&prod_icms["vBC"]),
            icms_picms: value_to_f64(&prod_icms["pICMS"]),
            icms_vicms: value_to_f64(&prod_icms["vICMS"]),
        }
    }
}
