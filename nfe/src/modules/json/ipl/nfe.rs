use crate::modules::json::structs::{dest::Dest, emit::Emit, nfe_struct::Nfe, produtos::Produto};
use utils::core::parser::{value_to_f64, value_to_string};
use xml_json::to_json_from_file;

impl Nfe {
    #[allow(dead_code)]
    pub fn empty() -> Nfe {
        Nfe {
            nfe_cdv: "Null".to_string(),
            nfe_cmunfg: "Null".to_string(),
            nfe_cnf: "Null".to_string(),
            nfe_cuf: "Null".to_string(),
            nfe_dhemi: "Null".to_string(),
            nfe_dhsaient: "Null".to_string(),
            nfe_finnfe: "Null".to_string(),
            nfe_nfe_iddest: "Null".to_string(),
            nfe_indfinal: "Null".to_string(),
            nfe_indintermed: "Null".to_string(),
            nfe_indpres: "Null".to_string(),
            nfe_modnfe: "Null".to_string(),
            nfe_nnf: "Null".to_string(),
            nfe_natop: "Null".to_string(),
            nfe_procemi: "Null".to_string(),
            nfe_serie: "Null".to_string(),
            nfe_tpamb: "Null".to_string(),
            nfe_tpemis: "Null".to_string(),
            nfe_tpimp: "Null".to_string(),
            nfe_tpnf: "Null".to_string(),
            nfe_verproc: "Null".to_string(),
            nfe_nftotal: 0.0,
            nfe_emit: Emit::empty(),
            nfe_dest: Dest::empty(),
            nfe_produtos: Produto::empty(),
        }
    }

    #[allow(dead_code)]
    pub fn new(file_path: &str) -> Nfe {
        let base = to_json_from_file(file_path).unwrap();
        let base_nfe = &base["nfeProc"]["NFe"]["infNFe"]["ide"];
        let value: Nfe = Nfe {
            nfe_cdv: value_to_string(&base_nfe["cDV"]),
            nfe_cmunfg: value_to_string(&base_nfe["cMunFG"]),
            nfe_cnf: value_to_string(&base_nfe["cNF"]),
            nfe_cuf: value_to_string(&base_nfe["cUF"]),
            nfe_dhemi: value_to_string(&base_nfe["dhEmi"]),
            nfe_dhsaient: value_to_string(&base_nfe["dhSaiEnt"]),
            nfe_finnfe: value_to_string(&base_nfe["finNFe"]),
            nfe_nfe_iddest: value_to_string(&base_nfe["idDest"]),
            nfe_indfinal: value_to_string(&base_nfe["indFinal"]),
            nfe_indintermed: value_to_string(&base_nfe["indIntermed"]),
            nfe_indpres: value_to_string(&base_nfe["indPres"]),
            nfe_modnfe: value_to_string(&base_nfe["mod"]),
            nfe_nnf: value_to_string(&base_nfe["nNF"]),
            nfe_natop: value_to_string(&base_nfe["natOp"]),
            nfe_procemi: value_to_string(&base_nfe["procEmi"]),
            nfe_serie: value_to_string(&base_nfe["serie"]),
            nfe_tpamb: value_to_string(&base_nfe["tpAmb"]),
            nfe_tpemis: value_to_string(&base_nfe["tpEmis"]),
            nfe_tpimp: value_to_string(&base_nfe["tpImp"]),
            nfe_tpnf: value_to_string(&base_nfe["tpNF"]),
            nfe_verproc: value_to_string(&base_nfe["verProc"]),
            nfe_nftotal: value_to_f64(&base["nfeProc"]["NFe"]["infNFe"]["total"]["ICMSTot"]["vNF"]),
            nfe_emit: Emit::new(&base),
            nfe_dest: Dest::new(&base),
            nfe_produtos: Produto::new(&base),
        };
        value
    }
}
