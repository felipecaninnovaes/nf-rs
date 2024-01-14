use crate::modules::json::structs::nfe::{NfeSelect, NfeJoinSelect, Permissions};
use sqlx::Row;
use uuid::Uuid;
use std::error::Error;

// get vec products id from a nfe
pub async fn get_products_id_from_nfe(
    pool: &sqlx::PgPool,
    nfeid: &i32,
) -> Result<Vec<i32>, Box<dyn Error>> {
    let q = "SELECT produto_idproduto FROM nfe_produto WHERE produto_idnfe = $1";
    let mut v: Vec<i32> = Vec::new();
    for row in sqlx::query(q).bind(nfeid).fetch_all(pool).await? {
        v.push(row.get(0));
    }
    Ok(v)
}

// get permissions
pub async fn get_permissions(
    pool: &sqlx::PgPool,
    user_id: Uuid) -> Result<Vec<Permissions>, Box<dyn Error>> {
        let allowed = true;
        let q_empresas_allowed = "SELECT permissions.permissions_user_id, permissions.permissions_allowed, empresas.cnpj from permissions INNER JOIN empresas ON permissions.permissions_empresa_id = empresas.idempresa where permissions.permissions_user_id = $1 and permissions.permissions_allowed = $2";
    
    
        let vec_cnpjs = sqlx::query_as::<_, Permissions>(q_empresas_allowed)
            .bind(user_id)
            .bind(allowed)
            .fetch_all(pool)
            .await?;
        Ok(vec_cnpjs)
    }  

// get all nfe
pub async fn all_nfe(pool: &sqlx::PgPool, user_id: Uuid) -> Result<String, Box<dyn Error>> {

    let vec_cnpjs = get_permissions(pool, user_id).await?;

    let mut result: Vec<NfeJoinSelect> = Vec::new(); // Change the type of result

    for row in vec_cnpjs {
        let q = "SELECT * FROM nfe INNER JOIN nfe_emit ON nfe.nfe_idemit = nfe_emit.emit_idemit INNER JOIN nfe_dest ON nfe.nfe_iddest = nfe_dest.dest_iddest WHERE nfe_emit.emit_cnpjcpf = $1 or nfe_dest.dest_cnpjcpf = $1";
        let res = sqlx::query_as::<_, NfeJoinSelect>(q).bind(row.cnpj).fetch_all(pool).await?;
            for row in res {
                result.push(row);
            }
         }
    
    let json = serde_json::to_string(&result)?;
    Ok(json)
}

// get nfe by emit
pub async fn nfe_by_emit(pool: &sqlx::PgPool, emit: &String) -> Result<String, Box<dyn Error>> {


    let q = "SELECT * FROM nfe INNER JOIN nfe_emit ON nfe.nfe_idemit = nfe_emit.emit_idemit INNER JOIN nfe_dest ON nfe.nfe_iddest = nfe_dest.dest_iddest WHERE nfe_emit.emit_cnpjcpf = $1";
    let res = sqlx::query_as::<_, NfeJoinSelect>(q)
        .bind(emit)
        .fetch_all(pool)
        .await?; // for row in sqlx::query(q).bind(emit).fetch_all(pool).await? {
    let json = serde_json::to_string(&res)?;
    Ok(json)
}

// get nfe by dest
pub async fn nfe_by_dest(pool: &sqlx::PgPool, dest: &i32) -> Result<String, Box<dyn Error>> {
    let q = "SELECT * FROM nfe WHERE nfe_iddest = $1";
    let res = sqlx::query_as::<_, NfeSelect>(q)
        .bind(dest)
        .fetch_all(pool)
        .await?; // for row in sqlx::query(q).bind(dest).fetch_all(pool).await? {
    let json = serde_json::to_string(&res)?;
    Ok(json)
}
