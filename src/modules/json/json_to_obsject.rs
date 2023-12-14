pub mod object {
    use std::io::Empty;

    use crate::modules::json::structs::objects::*;
    use serde_json::Value;

    impl Ipi {
        pub fn empty() -> Ipi {
            let value: Ipi = Ipi { c_enq: 0, cst: 0 };
            return value;
        }
    }

    impl Produto {
        pub fn new(file_path: &str) -> Vec<Produto> {
            use crate::modules::util::files_utils::utils::json_to_object;
            use crate::modules::util::parse_utils::utils::{
                parse_value_to_f64, parse_value_to_i64, parse_value_to_string,
            };
            let mut result: Vec<Produto> = Vec::new();
            let v = json_to_object(file_path.to_string());

            let mut i = 0;
            loop {
                let dest_cnpj = &v["nfeProc"]["NFe"][0]["infNFe"][0]["dest"][0]["CNPJ"][0];
                let dest_cpf = &v["nfeProc"]["NFe"][0]["infNFe"][0]["dest"][0]["CPF"][0];
                let prodid = &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["$"]["nItem"];
                let prod = &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["prod"][0];
                let prod_icms = &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["imposto"][0]
                    ["ICMS"][0]["ICMS00"][0];

                println!("{}", i);
                if prodid == &Value::Null {
                    break;
                } else {
                    let mut prod_ipi: Ipi = Ipi::empty();
                    if dest_cnpj != &Value::Null {
                        prod_ipi = Ipi {
                            c_enq: parse_value_to_i64(
                                &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["imposto"][0]["IPI"]
                                    [0]["cEnq"][0],
                            ),
                            cst: parse_value_to_i64(
                                &v["nfeProc"]["NFe"][0]["infNFe"][0]["det"][i]["imposto"][0]["IPI"]
                                    [0]["IPINT"][0]["CST"][0],
                            ),
                        };
                    } else {
                        prod_ipi = Ipi { c_enq: 0, cst: 0 };
                    }
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
                            v_icms: parse_value_to_f64(&prod_icms["vICMS"][0]),
                        },
                        ipi: prod_ipi,
                    };
                    result.push(produto);
                    i += 1;
                }
            }
            println!("{:?}", result[219]);
            return result;
        }
    }
}
