use crate::modules::json::structs::impostos::Pis;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;

impl Pis {
    #[allow(dead_code)]
    pub fn empty() -> Pis {
        Pis {
            pis_cst: 0,
            pis_vbc: 0.0,
            pis_ppis: 0.0,
            pis_vpis: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> Pis {
        let prod_pis = &base["imposto"]["PIS"]["PISAliq"];
        Pis {
            pis_cst: parse_value_to_i64(&prod_pis["CST"]),
            pis_vbc: parse_value_to_f64(&prod_pis["vBC"]),
            pis_ppis: parse_value_to_f64(&prod_pis["pPIS"]),
            pis_vpis: parse_value_to_f64(&prod_pis["vPIS"]),
        }
    }
}