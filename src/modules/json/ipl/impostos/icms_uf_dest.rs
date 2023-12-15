pub mod icms_uf_dest {
    use crate::modules::json::structs::impostos::impostos::IcmsUfDest;
    use crate::modules::util::parse_utils::utils::parse_value_to_f64;
    use serde_json::Value;

    impl IcmsUfDest {
        #[allow(dead_code)]
        pub fn empty() -> IcmsUfDest {
            let value: IcmsUfDest = IcmsUfDest {
                v_bcufdest: 0.0,
                v_bcfcpufdest: 0.0,
                p_fcpufdest: 0.0,
                p_icmsufdest: 0.0,
                p_icmsinter: 0.0,
                p_icmsinter_part: 0.0,
                v_fcpufdest: 0.0,
                v_icmsufdest: 0.0,
                v_icmsufremet: 0.0,
            };
            return value;
        }
        #[allow(dead_code)]
        pub fn new(base: &Value) -> IcmsUfDest {
            let prod_icms_uf_dest = &base["imposto"][0]["ICMSUFDest"][0];
            if prod_icms_uf_dest != &Value::Null {
                let value: IcmsUfDest = IcmsUfDest {
                    v_bcufdest: parse_value_to_f64(&prod_icms_uf_dest["vBCUFDest"][0]),
                    v_bcfcpufdest: parse_value_to_f64(&prod_icms_uf_dest["vBCFCPUFDest"][0]),
                    p_fcpufdest: parse_value_to_f64(&prod_icms_uf_dest["pFCPUFDest"][0]),
                    p_icmsufdest: parse_value_to_f64(&prod_icms_uf_dest["pICMSUFDest"][0]),
                    p_icmsinter: parse_value_to_f64(&prod_icms_uf_dest["pICMSInter"][0]),
                    p_icmsinter_part: parse_value_to_f64(&prod_icms_uf_dest["pICMSInterPart"][0]),
                    v_fcpufdest: parse_value_to_f64(&prod_icms_uf_dest["vFCPUFDest"][0]),
                    v_icmsufdest: parse_value_to_f64(&prod_icms_uf_dest["vICMSUFDest"][0]),
                    v_icmsufremet: parse_value_to_f64(&prod_icms_uf_dest["vICMSUFRemet"][0]),
                };
                return value;
            } else {
                return IcmsUfDest::empty();
            }
        }
    }
}
