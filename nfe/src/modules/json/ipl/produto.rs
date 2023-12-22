use crate::modules::json::structs::impostos::*;
use crate::modules::json::structs::produtos::Produto;
use serde_json::Value;

impl Produto {
    #[allow(dead_code)]
    pub fn empty() -> Vec<Produto> {
        vec![Produto {
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
        }]
    }

    pub fn new(base: &Value) -> Vec<Produto> {
        use crate::modules::util::parse_utils::{parse_value_to_f64, parse_value_to_string};
        let mut result: Vec<Produto> = Vec::new();
        let dest_cnpj = &base["nfeProc"]["NFe"]["infNFe"]["dest"]["CNPJ"];
        let det = &base["nfeProc"]["NFe"]["infNFe"]["det"];

        match det {
            Value::Array(dets) => {
                for det_item in dets {
                    if let Value::String(prodid) = &det_item["@nItem"] {
                        let prod = &det_item["prod"];
                        let produto = Produto {
                            n_item: prodid.to_string(),
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
                                icms: Icms::new(det_item),
                                ipi: Ipi::new(dest_cnpj, det_item),
                                pis: Pis::new(det_item),
                                cofins: Cofins::new(det_item),
                                icms_uf_dest: IcmsUfDest::new(det_item),
                            },
                        };
                        result.push(produto);
                    }
                }
            }
            _ => {
                let prod = &det["prod"];
                let produto = Produto {
                    n_item: parse_value_to_string(&det["@nItem"]),
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
                        icms: Icms::new(det),
                        ipi: Ipi::new(dest_cnpj, det),
                        pis: Pis::new(det),
                        cofins: Cofins::new(det),
                        icms_uf_dest: IcmsUfDest::new(det),
                    },
                };
                result.push(produto);
            }
        }
        result
    }
}