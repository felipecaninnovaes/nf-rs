use crate::modules::json::structs::impostos::Ipi;
use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_i64};
use serde_json::Value;
impl Ipi {
    #[allow(dead_code)]
    pub fn empty() -> Ipi {
        let value: Ipi = Ipi {
            c_enq: 0,
            cst: 0,
            v_bc: 0.0,
            p_ipi: 0,
            v_ipi: 0,
        };
        return value;
    }
    #[allow(dead_code)]
    pub fn new(dest_cnpj: &Value, base: &Value) -> Ipi {
        let ipi = &base["imposto"]["IPI"];
        let ipi_cst_trib = &ipi["IPITrib"];
        let ipi_cst = &ipi["IPINT"];

        match (dest_cnpj, ipi_cst_trib) {
            (&Value::Null, _) => Ipi::empty(),
            (_, &Value::Null) => Ipi {
                c_enq: parse_value_to_i64(&ipi["cEnq"]),
                cst: parse_value_to_i64(&ipi_cst["CST"]),
                v_bc: 0.0,
                p_ipi: 0,
                v_ipi: 0,
            },
            _ => Ipi {
                c_enq: parse_value_to_i64(&ipi["cEnq"]),
                cst: parse_value_to_i64(&ipi_cst_trib["CST"]),
                v_bc: parse_value_to_f64(&ipi_cst_trib["vBC"]),
                p_ipi: parse_value_to_i64(&ipi_cst_trib["pIPI"]),
                v_ipi: parse_value_to_i64(&ipi_cst_trib["vIPI"]),
            },
        }
    }
}
