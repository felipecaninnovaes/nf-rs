pub mod ipi {
    use crate::modules::json::structs::structs::objects::*;
    use crate::modules::util::parse_utils::utils::{parse_value_to_f64, parse_value_to_i64};
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
            let ipi = &base["imposto"][0]["IPI"][0];
            let ipi_cst_trib = &ipi["IPITrib"][0];
            let ipi_cst = &ipi["IPINT"][0];
            if dest_cnpj != &Value::Null {
                if ipi_cst_trib != &Value::Null {
                    let prod_ipi = Ipi {
                        c_enq: parse_value_to_i64(&ipi["cEnq"][0]),
                        cst: parse_value_to_i64(&ipi_cst_trib["CST"][0]),
                        v_bc: parse_value_to_f64(&ipi_cst_trib["vBC"][0]),
                        p_ipi: parse_value_to_i64(&ipi_cst_trib["pIPI"][0]),
                        v_ipi: parse_value_to_i64(&ipi_cst_trib["vIPI"][0]),
                    };
                    return prod_ipi;
                } else {
                    let prod_ipi = Ipi {
                        c_enq: parse_value_to_i64(&ipi["cEnq"][0]),
                        cst: parse_value_to_i64(&ipi_cst["CST"][0]),
                        v_bc: 0.0,
                        p_ipi: 0,
                        v_ipi: 0,
                    };
                    return prod_ipi;
                }
            } else {
                let prod_ipi = Ipi {
                    c_enq: 0,
                    cst: 0,
                    p_ipi: 0,
                    v_bc: 0.0,
                    v_ipi: 0,
                };
                return prod_ipi;
            }
        }
    }
}
