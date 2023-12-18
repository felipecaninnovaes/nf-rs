use xml_json::to_json_from_file;

use crate::modules::json::structs::{dest::Dest, emit::Emit, nfe::Nfe, produtos::Produto};

impl Nfe {
    #[allow(dead_code)]
    pub fn empty() -> Nfe {
        let value: Nfe = Nfe {
            c_dv: 0,
            c_mun_fg: 0,
            c_nf: 0,
            c_uf: 0,
            dh_emi: "".to_string(),
            dh_sai_ent: "".to_string(),
            fin_nfe: 0,
            id_dest: 0,
            ind_final: 0,
            ind_intermed: 0,
            ind_pres: 0,
            mod_nfe: 0,
            n_nf: 0,
            nat_op: "".to_string(),
            proc_emi: 0,
            serie: 0,
            tp_amb: 0,
            tp_emis: 0,
            tp_imp: 0,
            tp_nf: 0,
            ver_proc: "".to_string(),
            emit: Emit::empty(),
            dest: Dest::empty(),
            produtos: Produto::empty(),
        };
        return value;
    }

    #[allow(dead_code)]
    pub fn new(file_path: &str) -> Nfe {
        use crate::modules::util::parse_utils::{
            parse_value_to_i32, parse_value_to_i64, parse_value_to_string,
        };
        let base = to_json_from_file(file_path).unwrap();
        let base_nfe = &base["nfeProc"]["NFe"]["infNFe"]["ide"];
        let value: Nfe = Nfe {
            c_dv: parse_value_to_i64(&base_nfe["cDV"]),
            c_mun_fg: parse_value_to_i64(&base_nfe["cMunFG"]),
            c_nf: parse_value_to_i64(&base_nfe["cNF"]),
            c_uf: parse_value_to_i64(&base_nfe["cUF"]),
            dh_emi: parse_value_to_string(&base_nfe["dhEmi"]),
            dh_sai_ent: parse_value_to_string(&base_nfe["dhSaiEnt"]),
            fin_nfe: parse_value_to_i32(&base_nfe["finNFe"]),
            id_dest: parse_value_to_i32(&base_nfe["idDest"]),
            ind_final: parse_value_to_i32(&base_nfe["indFinal"]),
            ind_intermed: parse_value_to_i32(&base_nfe["indIntermed"]),
            ind_pres: parse_value_to_i32(&base_nfe["indPres"]),
            mod_nfe: parse_value_to_i32(&base_nfe["mod"]),
            n_nf: parse_value_to_i64(&base_nfe["nNF"]),
            nat_op: parse_value_to_string(&base_nfe["natOp"]),
            proc_emi: parse_value_to_i32(&base_nfe["procEmi"]),
            serie: parse_value_to_i32(&base_nfe["serie"]),
            tp_amb: parse_value_to_i32(&base_nfe["tpAmb"]),
            tp_emis: parse_value_to_i32(&base_nfe["tpEmis"]),
            tp_imp: parse_value_to_i32(&base_nfe["tpImp"]),
            tp_nf: parse_value_to_i32(&base_nfe["tpNF"]),
            ver_proc: parse_value_to_string(&base_nfe["verProc"]),
            emit: Emit::new(&base),
            dest: Dest::new(&base),
            produtos: Produto::new(&base),
        };
        return value;
    }
}
