use crate::modules::json::structs::impostos::IcmsUfDest;
use crate::modules::util::parse_utils::parse_value_to_f64;
use serde_json::Value;

impl IcmsUfDest {
    #[allow(dead_code)]
    pub fn empty() -> IcmsUfDest {
        IcmsUfDest {
            icms_uf_vbcufdest: 0.0,
            icms_uf_vbcfcpufdest: 0.0,
            icms_uf_pfcpufdest: 0.0,
            icms_uf_picmsufdest: 0.0,
            icms_uf_picmsinter: 0.0,
            icms_uf_picmsinterpart: 0.0,
            icms_uf_vfcpufdest: 0.0,
            icms_uf_vicmsufdest: 0.0,
            icms_uf_vicmsufremet: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn new(base: &Value) -> IcmsUfDest {
        match &base["imposto"]["ICMSUFDest"] {
            Value::Null => IcmsUfDest::empty(),
            prod_icms_uf_dest => IcmsUfDest {
                icms_uf_vbcufdest: parse_value_to_f64(&prod_icms_uf_dest["vBCUFDest"]),
                icms_uf_vbcfcpufdest: parse_value_to_f64(&prod_icms_uf_dest["vBCFCPUFDest"]),
                icms_uf_pfcpufdest: parse_value_to_f64(&prod_icms_uf_dest["pFCPUFDest"]),
                icms_uf_picmsufdest: parse_value_to_f64(&prod_icms_uf_dest["pICMSUFDest"]),
                icms_uf_picmsinter: parse_value_to_f64(&prod_icms_uf_dest["pICMSInter"]),
                icms_uf_picmsinterpart: parse_value_to_f64(&prod_icms_uf_dest["pICMSInterPart"]),
                icms_uf_vfcpufdest: parse_value_to_f64(&prod_icms_uf_dest["vFCPUFDest"]),
                icms_uf_vicmsufdest: parse_value_to_f64(&prod_icms_uf_dest["vICMSUFDest"]),
                icms_uf_vicmsufremet: parse_value_to_f64(&prod_icms_uf_dest["vICMSUFRemet"]),
            }
        }
    }
}
