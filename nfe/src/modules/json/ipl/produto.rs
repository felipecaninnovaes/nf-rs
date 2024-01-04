use crate::modules::json::structs::impostos::*;
use crate::modules::json::structs::produtos::Produto;
use serde_json::Value;

impl Produto {
    #[allow(dead_code)]
    pub fn empty() -> Vec<Produto> {
        vec![Produto {
            produto_nitem: "Null".to_string(),
            produto_cprod: "Null".to_string(),
            produto_cean: "Null".to_string(),
            produto_xprod: "Null".to_string(),
            produto_ncm: "Null".to_string(),
            produto_cfop: "Null".to_string(),
            produto_ucom: "Null".to_string(),
            produto_qcom: 0.0,
            produto_vuncom: 0.0,
            produto_vprod: 0.0,
            produto_ceantrib: "Null".to_string(),
            produto_utrib: "Null".to_string(),
            produto_qtrib: 0.0,
            produto_vuntrib: 0.0,
            produto_indtot: "Null".to_string(),
            produto_xped: "Null".to_string(),
            imposto: Imposto {
                imposto_icms: Icms::empty(),
                imposto_ipi: Ipi::empty(),
                imposto_pis: Pis::empty(),
                imposto_cofins: Cofins::empty(),
                imposto_icms_uf_dest: IcmsUfDest::empty(),
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
                            produto_nitem: prodid.to_string(),
                            produto_cprod: parse_value_to_string(&prod["cProd"]),
                            produto_cean: parse_value_to_string(&prod["cEAN"]),
                            produto_xprod: parse_value_to_string(&prod["xProd"]),
                            produto_ncm: parse_value_to_string(&prod["NCM"]),
                            produto_cfop: parse_value_to_string(&prod["CFOP"]),
                            produto_ucom: parse_value_to_string(&prod["uCom"]),
                            produto_qcom: parse_value_to_f64(&prod["qCom"]),
                            produto_vuncom: parse_value_to_f64(&prod["vUnCom"]),
                            produto_vprod: parse_value_to_f64(&prod["vProd"]),
                            produto_ceantrib: parse_value_to_string(&prod["cEANTrib"]),
                            produto_utrib: parse_value_to_string(&prod["uTrib"]),
                            produto_qtrib: parse_value_to_f64(&prod["qTrib"]),
                            produto_vuntrib: parse_value_to_f64(&prod["vUnTrib"]),
                            produto_indtot: parse_value_to_string(&prod["indTot"]),
                            produto_xped: parse_value_to_string(&prod["xPed"]),
                            imposto: Imposto {
                                imposto_icms: Icms::new(det_item),
                                imposto_ipi: Ipi::new(dest_cnpj, det_item),
                                imposto_pis: Pis::new(det_item),
                                imposto_cofins: Cofins::new(det_item),
                                imposto_icms_uf_dest: IcmsUfDest::new(det_item),
                            },
                        };
                        result.push(produto);
                    }
                }
            }
            _ => {
                let prod = &det["prod"];
                let produto = Produto {
                    produto_nitem: parse_value_to_string(&det["@nItem"]),
                    produto_cprod: parse_value_to_string(&prod["cProd"]),
                    produto_cean: parse_value_to_string(&prod["cEAN"]),
                    produto_xprod: parse_value_to_string(&prod["xProd"]),
                    produto_ncm: parse_value_to_string(&prod["NCM"]),
                    produto_cfop: parse_value_to_string(&prod["CFOP"]),
                    produto_ucom: parse_value_to_string(&prod["uCom"]),
                    produto_qcom: parse_value_to_f64(&prod["qCom"]),
                    produto_vuncom: parse_value_to_f64(&prod["vUnCom"]),
                    produto_vprod: parse_value_to_f64(&prod["vProd"]),
                    produto_ceantrib: parse_value_to_string(&prod["cEANTrib"]),
                    produto_utrib: parse_value_to_string(&prod["uTrib"]),
                    produto_qtrib: parse_value_to_f64(&prod["qTrib"]),
                    produto_vuntrib: parse_value_to_f64(&prod["vUnTrib"]),
                    produto_indtot: parse_value_to_string(&prod["indTot"]),
                    produto_xped: parse_value_to_string(&prod["xPed"]),
                    imposto: Imposto {
                        imposto_icms: Icms::new(det),
                        imposto_ipi: Ipi::new(dest_cnpj, det),
                        imposto_pis: Pis::new(det),
                        imposto_cofins: Cofins::new(det),
                        imposto_icms_uf_dest: IcmsUfDest::new(det),
                    },
                };
                result.push(produto);
            }
        }
        result
    }
}
