pub mod icms {
    use crate::modules::json::structs::structs::objects::*;
    use crate::modules::util::parse_utils::utils::{parse_value_to_f64, parse_value_to_i64};
    use serde_json::Value;
    impl Icms {
        #[allow(dead_code)]
        pub fn empty() -> Icms {
            let value: Icms = Icms { orig: 0, cst: 0, mod_bc: 0, v_bc: 0.0, p_icms: 0.0, v_icms: 0.0 };
            return value;
        }
        #[allow(dead_code)]
        pub fn new(base: &Value) -> Icms {
            let prod_icms = &base["imposto"][0]["ICMS"][0]["ICMS00"][0];
            let value: Icms = Icms {
                orig: parse_value_to_i64(&prod_icms["orig"][0]),
                cst: parse_value_to_i64(&prod_icms["CST"][0]),
                mod_bc: parse_value_to_i64(&prod_icms["modBC"][0]),
                v_bc: parse_value_to_f64(&prod_icms["vBC"][0]),
                p_icms: parse_value_to_f64(&prod_icms["pICMS"][0]),
                v_icms: parse_value_to_f64(&prod_icms["vICMS"][0]),
            };
            return value;
        }
    }
}