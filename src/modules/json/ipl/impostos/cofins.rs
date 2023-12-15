use crate::modules::json::structs::impostos::Cofins;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;
impl Cofins {
    #[allow(dead_code)]
    pub fn empty() -> Cofins {
        let value: Cofins = Cofins {
            cst: 0,
            v_bc: 0.0,
            p_cofins: 0.0,
            v_cofins: 0.0,
        };
        return value;
    }
    #[allow(dead_code)]
    pub fn new(base: &Value) -> Cofins {
        let prod_cofins = &base["imposto"][0]["COFINS"][0]["COFINSAliq"][0];
        let value: Cofins = Cofins {
            cst: parse_value_to_i64(&prod_cofins["CST"][0]),
            v_bc: parse_value_to_f64(&prod_cofins["vBC"][0]),
            p_cofins: parse_value_to_f64(&prod_cofins["pCOFINS"][0]),
            v_cofins: parse_value_to_f64(&prod_cofins["vCOFINS"][0]),
        };
        return value;
    }
}
