use crate::modules::json::structs::impostos::*;
use crate::modules::json::structs::produtos::Produto;
use serde_json::Value;

use xml_json::to_json_from_file;

impl Produto {
    #[allow(dead_code)]
    pub fn empty() -> Vec<Produto> {
        let value: Produto = Produto {
            n_item: 0,
            c_prod: "".to_string(),
            c_ean: "".to_string(),
            x_prod: "".to_string(),
            ncm: 0,
            cfop: 0,
            u_com: "".to_string(),
            q_com: 0.0,
            v_un_com: 0.0,
            v_prod: 0.0,
            c_eantrib: "".to_string(),
            u_trib: "".to_string(),
            q_trib: 0.0,
            v_un_trib: 0.0,
            ind_tot: 0,
            x_ped: "".to_string(),
            impostos: Impostos {
                icms: Icms::empty(),
                ipi: Ipi::empty(),
                pis: Pis::empty(),
                cofins: Cofins::empty(),
                icms_uf_dest: IcmsUfDest::empty(),
            },
        };
        return vec![value];
    }

    pub fn new(file_path: &str) -> Vec<Produto> {
        use crate::modules::util::parse_utils::{
            parse_value_to_f64, parse_value_to_i64, parse_value_to_string,
        };
        let mut result: Vec<Produto> = Vec::new();
        let v = to_json_from_file(file_path).unwrap();

        let mut i = 0;
        loop {
            let base = &v["nfeProc"]["NFe"]["infNFe"]["det"][i];
            let prodid = &base["@nItem"];
            if prodid == &Value::Null {
                break;
            } else {
                let dest_cnpj = &v["nfeProc"]["NFe"]["infNFe"]["dest"]["CNPJ"];
                let prod = &base["prod"];
                let produto = Produto {
                    n_item: parse_value_to_i64(&prodid),
                    c_prod: parse_value_to_string(&prod["cProd"]),
                    c_ean: parse_value_to_string(&prod["cEAN"]),
                    x_prod: parse_value_to_string(&prod["xProd"]),
                    ncm: parse_value_to_i64(&prod["NCM"]),
                    cfop: parse_value_to_i64(&prod["CFOP"]),
                    u_com: parse_value_to_string(&prod["uCom"]),
                    q_com: parse_value_to_f64(&prod["qCom"]),
                    v_un_com: parse_value_to_f64(&prod["vUnCom"]),
                    v_prod: parse_value_to_f64(&prod["vProd"]),
                    c_eantrib: parse_value_to_string(&prod["cEANTrib"]),
                    u_trib: parse_value_to_string(&prod["uTrib"]),
                    q_trib: parse_value_to_f64(&prod["qTrib"]),
                    v_un_trib: parse_value_to_f64(&prod["vUnTrib"]),
                    ind_tot: parse_value_to_i64(&prod["indTot"]),
                    x_ped: parse_value_to_string(&prod["xPed"]),
                    impostos: Impostos {
                        icms: Icms::new(base),
                        ipi: Ipi::new(dest_cnpj, base),
                        pis: Pis::new(base),
                        cofins: Cofins::new(base),
                        icms_uf_dest: IcmsUfDest::new(base),
                    },
                };
                result.push(produto);
                i += 1;
            }
        }
        return result;
    }
}
