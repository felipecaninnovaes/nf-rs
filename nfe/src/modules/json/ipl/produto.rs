use crate::modules::json::structs::impostos::*;
use crate::modules::json::structs::produtos::Produto;
use serde_json::Value;

impl Produto {
    #[allow(dead_code)]
    pub fn empty() -> Vec<Produto> {
        let value: Produto = Produto {
            n_item: "Null".to_string(),
            c_prod: "Null".to_string(),
            c_ean: "Null".to_string(),
            x_prod: "Null".to_string(),
            ncm: "Null".to_string(),
            cfop: "Null".to_string(),
            u_com: "Null".to_string(),
            q_com: 0.0,
            v_un_com: 0.0,
            v_prod: 0.0,
            c_eantrib: "Null".to_string(),
            u_trib: "Null".to_string(),
            q_trib: 0.0,
            v_un_trib: 0.0,
            ind_tot: "Null".to_string(),
            x_ped: "Null".to_string(),
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

    pub fn new(base: &Value) -> Vec<Produto> {
        use crate::modules::util::parse_utils::{
            parse_value_to_f64, parse_value_to_string,
        };
        let mut result: Vec<Produto> = Vec::new();
        let mut i = 0;
        let dest_cnpj = &base["nfeProc"]["NFe"]["infNFe"]["dest"]["CNPJ"];

        loop {
            let base_prod = &base["nfeProc"]["NFe"]["infNFe"]["det"][i];
            let prodid = &base_prod["@nItem"];
            if prodid == &Value::Null {
                break;
            } else {
                let prod = &base_prod["prod"];
                let produto = Produto {
                    n_item: parse_value_to_string(&prodid),
                    c_prod: parse_value_to_string(&prod["cProd"]),
                    c_ean: parse_value_to_string(&prod["cEAN"]),
                    x_prod: parse_value_to_string(&prod["xProd"]),
                    ncm: parse_value_to_string(&prod["NCM"]),
                    cfop: parse_value_to_string(&prod["CFOP"]),
                    u_com: parse_value_to_string(&prod["uCom"]),
                    q_com: parse_value_to_f64(&prod["qCom"]),
                    v_un_com: parse_value_to_f64(&prod["vUnCom"]),
                    v_prod: parse_value_to_f64(&prod["vProd"]),
                    c_eantrib: parse_value_to_string(&prod["cEANTrib"]),
                    u_trib: parse_value_to_string(&prod["uTrib"]),
                    q_trib: parse_value_to_f64(&prod["qTrib"]),
                    v_un_trib: parse_value_to_f64(&prod["vUnTrib"]),
                    ind_tot: parse_value_to_string(&prod["indTot"]),
                    x_ped: parse_value_to_string(&prod["xPed"]),
                    impostos: Impostos {
                        icms: Icms::new(base_prod),
                        ipi: Ipi::new(dest_cnpj, base_prod),
                        pis: Pis::new(base_prod),
                        cofins: Cofins::new(base_prod),
                        icms_uf_dest: IcmsUfDest::new(base_prod),
                    },
                };
                result.push(produto);
                i += 1;
            }
        }
        return result;
    }
}
