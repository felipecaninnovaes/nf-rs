use std::error::Error;

use nfe::modules::json::structs::dest::DestId;

pub async fn select_dest_id(
    pool: &sqlx::PgPool,
    cnpj_cpf: &String,
) -> Result<DestId, Box<dyn Error>> {
    let q = "SELECT id FROM nfe_dest WHERE dest_cnpjcpf = $1";
    let query: DestId = sqlx::query_as::<_, DestId>(q)
        .bind(cnpj_cpf)
        .fetch_one(pool)
        .await?;
    match query.dest_iddest {
        0 => Err("emit_idemit not found".into()),
        _ => Ok(query),
    }
}
