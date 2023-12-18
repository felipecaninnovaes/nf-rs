use xml_json::to_json_from_file;

use crate::modules::json::structs::{nfe::Nfe, emit::Emit, dest::Dest, produtos::Produto};



impl Nfe {

    pub fn empty() -> Nfe {
        let value: Nfe = Nfe { c_dv: 0, c_mun_fg: 0, c_nf: 0, c_uf: 0, dh_emi: "".to_string(), dh_sai_ent: "".to_string(), fin_nfe: 0, id_dest: 0, ind_final: 0, ind_intermed: 0, ind_pres: 0, mod_nfe: 0, n_nf: 0, nat_op: "".to_string(), proc_emi: 0, serie: 0, tp_amb: 0, tp_emis: 0, tp_imp: 0, tp_nf: 0, ver_proc: "".to_string(), emit: Emit::empty(), dest: Dest::empty(), produtos: Produto::empty() };
        return value;
    }

    // pub fn new(file_path: &str) -> Nfe{
    // use crate::modules::util::parse_utils::{
    //     parse_value_to_f64, parse_value_to_i64, parse_value_to_string,
    // };
    // let mut result: Vec<Produto> = Vec::new();
    // let v = to_json_from_file(file_path).unwrap();

    // let mut i = 0;
    // let dest_cnpj = &v["nfeProc"]["NFe"]["infNFe"]["dest"]["CNPJ"];
    // }
}