use crate::modules::json::structs::impostos::Pis;
use serde_json::Value;
use utils::core::parser::{value_to_f64, value_to_i64};

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
            pis_cst: value_to_i64(&prod_pis["CST"]),
            pis_vbc: value_to_f64(&prod_pis["vBC"]),
            pis_ppis: value_to_f64(&prod_pis["pPIS"]),
            pis_vpis: value_to_f64(&prod_pis["vPIS"]),
        }
    }
}
