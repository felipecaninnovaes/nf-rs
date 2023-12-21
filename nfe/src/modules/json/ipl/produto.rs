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
        use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_string};
        let mut result: Vec<Produto> = Vec::new();
        let mut i = 0;
        let dest_cnpj = &base["nfeProc"]["NFe"]["infNFe"]["dest"]["CNPJ"];

        loop {
            let base_prod_mult = &base["nfeProc"]["NFe"]["infNFe"]["det"][i];
            let prodid_mult = &base_prod_mult["@nItem"];
            let base_prod_single = &base["nfeProc"]["NFe"]["infNFe"]["det"];
            let prodid_single = &base_prod_single["@nItem"];
            if prodid_mult != &Value::Null {
                let prod = &base_prod_mult["prod"];
                let produto = Produto {
                    n_item: parse_value_to_string(&prodid_mult),
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
                        icms: Icms::new(base_prod_mult),
                        ipi: Ipi::new(dest_cnpj, base_prod_mult),
                        pis: Pis::new(base_prod_mult),
                        cofins: Cofins::new(base_prod_mult),
                        icms_uf_dest: IcmsUfDest::new(base_prod_mult),
                    },
                };
                result.push(produto);
                i += 1;
            } else if prodid_single != &Value::Null {
                println!("prodid_single: {}", prodid_single);
                let prod = &base_prod_single["prod"];
                let produto = Produto {
                    n_item: parse_value_to_string(&prodid_single),
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
                        icms: Icms::new(base_prod_single),
                        ipi: Ipi::new(dest_cnpj, base_prod_single),
                        pis: Pis::new(base_prod_single),
                        cofins: Cofins::new(base_prod_single),
                        icms_uf_dest: IcmsUfDest::new(base_prod_single),
                    },
                };
                result.push(produto);
                break;
            } else {
                break;
            }
        }
        return result;
    }
}
