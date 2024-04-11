use crate::modules::json::structs::impostos::Ipi;
use serde_json::Value;
use utils::core::parser::{value_to_f64, value_to_i64};
impl Ipi {
    #[allow(dead_code)]
    pub fn empty() -> Ipi {
        Ipi {
            ipi_cenq: 0,
            ipi_cst: 0,
            ipi_vbc: 0.0,
            ipi_pipi: 0,
            ipi_vipi: 0,
        }
    }
    #[allow(dead_code)]
    pub fn new(dest_cnpj: &Value, base: &Value) -> Ipi {
        let ipi = &base["imposto"]["IPI"];
        let ipi_cst_trib = &ipi["IPITrib"];
        let ipi_cst = &ipi["IPINT"];

        match (dest_cnpj, ipi_cst_trib) {
            (&Value::Null, _) => Ipi::empty(),
            (_, &Value::Null) => Ipi {
                ipi_cenq: value_to_i64(&ipi["cEnq"]),
                ipi_cst: value_to_i64(&ipi_cst["CST"]),
                ipi_vbc: 0.0,
                ipi_pipi: 0,
                ipi_vipi: 0,
            },
            _ => Ipi {
                ipi_cenq: value_to_i64(&ipi["cEnq"]),
                ipi_cst: value_to_i64(&ipi_cst_trib["CST"]),
                ipi_vbc: value_to_f64(&ipi_cst_trib["vBC"]),
                ipi_pipi: value_to_i64(&ipi_cst_trib["pIPI"]),
                ipi_vipi: value_to_i64(&ipi_cst_trib["vIPI"]),
            },
        }
    }
}
