use xml_json::to_json_from_file;

use crate::modules::{json::structs::{dest::Dest, emit::Emit, nfe::Nfe, produtos::Produto}, util::parse_utils::parse_value_to_f64};

impl Nfe {
    #[allow(dead_code)]
    pub fn empty() -> Nfe {
        Nfe {
            c_dv: "Null".to_string(),
            c_mun_fg: "Null".to_string(),
            c_nf: "Null".to_string(),
            c_uf: "Null".to_string(),
            dh_emi: "Null".to_string(),
            dh_sai_ent: "Null".to_string(),
            fin_nfe: "Null".to_string(),
            id_dest: "Null".to_string(),
            ind_final: "Null".to_string(),
            ind_intermed: "Null".to_string(),
            ind_pres: "Null".to_string(),
            mod_nfe: "Null".to_string(),
            n_nf: "Null".to_string(),
            nat_op: "Null".to_string(),
            proc_emi: "Null".to_string(),
            serie: "Null".to_string(),
            tp_amb: "Null".to_string(),
            tp_emis: "Null".to_string(),
            tp_imp: "Null".to_string(),
            tp_nf: "Null".to_string(),
            ver_proc: "Null".to_string(),
            nf_total: 0.0,
            emit: Emit::empty(),
            dest: Dest::empty(),
            produtos: Produto::empty(),
        }
    }

    #[allow(dead_code)]
    pub fn new(file_path: &str) -> Nfe {
        use crate::modules::util::parse_utils::parse_value_to_string;

        let base = to_json_from_file(file_path).unwrap();
        let base_nfe = &base["nfeProc"]["NFe"]["infNFe"]["ide"];
        let value: Nfe = Nfe {
            c_dv: parse_value_to_string(&base_nfe["cDV"]),
            c_mun_fg: parse_value_to_string(&base_nfe["cMunFG"]),
            c_nf: parse_value_to_string(&base_nfe["cNF"]),
            c_uf: parse_value_to_string(&base_nfe["cUF"]),
            dh_emi: parse_value_to_string(&base_nfe["dhEmi"]),
            dh_sai_ent: parse_value_to_string(&base_nfe["dhSaiEnt"]),
            fin_nfe: parse_value_to_string(&base_nfe["finNFe"]),
            id_dest: parse_value_to_string(&base_nfe["idDest"]),
            ind_final: parse_value_to_string(&base_nfe["indFinal"]),
            ind_intermed: parse_value_to_string(&base_nfe["indIntermed"]),
            ind_pres: parse_value_to_string(&base_nfe["indPres"]),
            mod_nfe: parse_value_to_string(&base_nfe["mod"]),
            n_nf: parse_value_to_string(&base_nfe["nNF"]),
            nat_op: parse_value_to_string(&base_nfe["natOp"]),
            proc_emi: parse_value_to_string(&base_nfe["procEmi"]),
            serie: parse_value_to_string(&base_nfe["serie"]),
            tp_amb: parse_value_to_string(&base_nfe["tpAmb"]),
            tp_emis: parse_value_to_string(&base_nfe["tpEmis"]),
            tp_imp: parse_value_to_string(&base_nfe["tpImp"]),
            tp_nf: parse_value_to_string(&base_nfe["tpNF"]),
            ver_proc: parse_value_to_string(&base_nfe["verProc"]),
            nf_total: parse_value_to_f64(&base["nfeProc"]["NFe"]["infNFe"]["total"]["ICMSTot"]["vNF"]),
            emit: Emit::new(&base),
            dest: Dest::new(&base),
            produtos: Produto::new(&base),
        };
        value
    }
}