pub mod object {

    use serde_json::Value;

    use crate::modules::json::structs::objects::*;
    impl Produto {
        pub fn new(file_path: &str) -> Vec<Produto> {
            use crate::modules::util::files_utils::utils::json_to_object;
            use crate::modules::util::parse_utils::utils::{
                parse_value_to_f64, parse_value_to_i64, parse_value_to_string,
            };
            let mut result:Vec<Produto> = Vec::new();
            let v = json_to_object(file_path.to_string());
            
            let mut i = 0;
            loop {
                let prodid = &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["$"]["nItem"];
                let prod = &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["prod"][0];
                let prod_icms = &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["imposto"][0]["ICMS"][0]["ICMS00"][0];
                if prodid == &Value::Null {
                    break;
                } else {
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
                            icms: Icms { 
                                orig: parse_value_to_i64(&prod_icms["orig"][0]), 
                                cst: parse_value_to_i64(&prod_icms["CST"][0]), 
                                mod_bc: parse_value_to_i64(&prod_icms["modBC"][0]), 
                                v_bc: parse_value_to_f64(&prod_icms["vBC"][0]), 
                                p_icms: parse_value_to_f64(&prod_icms["pICMS"][0]), 
                                v_icms: parse_value_to_f64(&prod_icms["vICMS"][0])
                            }
                        };
                        result.push(produto);
                    i += 1;
                }
            }
            println!("{:?}", result);
            return result;
        }
    }
}
