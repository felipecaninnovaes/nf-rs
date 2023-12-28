
use sqlx::Row;
use std::error::Error;
use crate::modules::json::structs::nfe::NfeSelect;




// get vec products id from a nfe
pub async fn get_products_id_from_nfe(pool: &sqlx::PgPool, nfeid: &i32) -> Result<Vec<i32>, Box<dyn Error>> {
    let q = "SELECT idproduto FROM produto WHERE nfeidnfe = $1";
    let mut v: Vec<i32> = Vec::new();
    for row in sqlx::query(q).bind(nfeid).fetch_all(pool).await? {
        v.push(row.get(0));
    }
    Ok(v)
}

// get all nfe
pub async fn get_all_nfe(pool: &sqlx::PgPool) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe";
    let mut v: Vec<NfeSelect> = Vec::new();

    for row in sqlx::query(q).fetch_all(pool).await? {
        let row: NfeSelect = NfeSelect {
            nfeidnfe: row.get(0),
            c_dv: row.get(1),
            c_mun_fg: row.get(2),
            c_nf: row.get(3),
            c_uf: row.get(4),
            dh_emi: row.get(5),
            dh_sai_ent: row.get(6),
            fin_nfe: row.get(7),
            id_dest: row.get(8),
            ind_final: row.get(9),
            ind_intermed: row.get(10),
            ind_pres: row.get(11),
            mod_nfe: row.get(12),
            n_nf: row.get(13),
            nat_op: row.get(14),
            proc_emi: row.get(15),
            serie: row.get(16),
            tp_amb: row.get(17),
            tp_emis: row.get(18),
            tp_imp: row.get(19),
            tp_nf: row.get(20),
            ver_proc: row.get(21),
            nf_total: row.get(22),
            emit: row.get(23),
            dest: row.get(24),
        };
        v.push(row);
    }
    let json = serde_json::to_string(&v)?;
    Ok(json)
}