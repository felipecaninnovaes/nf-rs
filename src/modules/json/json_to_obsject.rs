pub mod object {

    use crate::modules::json::structs::structs::objects::Produto;
    use crate::modules::json::structs::impostos::impostos::{*};
    use serde_json::Value;

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
            use crate::modules::util::files_utils::utils::json_to_object;
            use crate::modules::util::parse_utils::utils::{
                parse_value_to_f64, parse_value_to_i64, parse_value_to_string,
            };
            let mut result: Vec<Produto> = Vec::new();
            let v = json_to_object(file_path.to_string());

            let mut i = 0;
            loop {
                let base = &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i];
                let prodid = &base["$"]["nItem"];
                if prodid == &Value::Null {
                    break;
                } else {
                    let dest_cnpj = &v["nfeProc"]["NFe"][0]["infNFe"][0]["dest"][0]["CNPJ"][0];
                    let prod = &base["prod"][0];
                    let produto = Produto {
                        n_item: parse_value_to_i64(&prodid),
                        c_prod: parse_value_to_string(&prod["cProd"][0]),
                        c_ean: parse_value_to_string(&prod["cEAN"][0]),
                        x_prod: parse_value_to_string(&prod["xProd"][0]),
                        ncm: parse_value_to_i64(&prod["NCM"][0]),
                        cfop: parse_value_to_i64(&prod["CFOP"][0]),
                        u_com: parse_value_to_string(&prod["uCom"][0]),
                        q_com: parse_value_to_f64(&prod["qCom"][0]),
                        v_un_com: parse_value_to_f64(&prod["vUnCom"][0]),
                        v_prod: parse_value_to_f64(&prod["vProd"][0]),
                        c_eantrib: parse_value_to_string(&prod["cEANTrib"][0]),
                        u_trib: parse_value_to_string(&prod["uTrib"][0]),
                        q_trib: parse_value_to_f64(&prod["qTrib"][0]),
                        v_un_trib: parse_value_to_f64(&prod["vUnTrib"][0]),
                        ind_tot: parse_value_to_i64(&prod["indTot"][0]),
                        x_ped: parse_value_to_string(&prod["xPed"][0]),
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
}
