use std::error::Error;

use nfe::modules::json::structs::emit::EmitId;

pub async fn select_emit_id(
    pool: &sqlx::PgPool,
    cnpj_cpf: &String,
) -> Result<EmitId, Box<dyn Error>> {
    let q = "SELECT id FROM nfe_emit WHERE emit_cnpjcpf = $1";
    let query = sqlx::query_as::<_, EmitId>(q)
        .bind(cnpj_cpf)
        .fetch_one(pool)
        .await?;
    match query.emit_idemit {
        0 => Err("emit_idemit not found".into()),
        _ => Ok(query),
    }
}
