use std::error::Error;

use super::select::get_products_id_from_nfe;

// delete ender
#[allow(dead_code)]
pub async fn delete_ender(
    pool: &sqlx::PgPool,
    nro: &String,
    cep: &String,
) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_ender WHERE ender_nro = $1 AND cep = $2";
    sqlx::query(q).bind(nro).bind(cep).execute(pool).await?;

    Ok(())
}

// delete emit
#[allow(dead_code)]
pub async fn delete_emit(
    pool: &sqlx::PgPool,
    cnpjcpf: &String,
    enderidender: &String,
) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_emit WHERE emit_cnpjcpf = $1 AND emit_idender = $2";
    sqlx::query(q)
        .bind(cnpjcpf)
        .bind(enderidender)
        .execute(pool)
        .await?;

    Ok(())
}

// delete dest
#[allow(dead_code)]
pub async fn delete_dest(
    pool: &sqlx::PgPool,
    cnpjcpf: &String,
    enderidender: &String,
) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_dest WHERE dest_cnpjcpf = $1 AND dest_idender = $2";
    sqlx::query(q)
        .bind(cnpjcpf)
        .bind(enderidender)
        .execute(pool)
        .await?;

    Ok(())
}

// delete pis
#[allow(dead_code)]
pub async fn delete_pis(pool: &sqlx::PgPool, prodid: &i32) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_pis WHERE pis_idproduto = $1";
    sqlx::query(q).bind(prodid).execute(pool).await?;
    Ok(())
}

// delete cofins
#[allow(dead_code)]
pub async fn delete_cofins(pool: &sqlx::PgPool, prodid: &i32) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_cofins WHERE cofins_idproduto = $1";
    sqlx::query(q).bind(prodid).execute(pool).await?;
    Ok(())
}

// delete ipi
#[allow(dead_code)]
pub async fn delete_ipi(pool: &sqlx::PgPool, prodid: &i32) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_ipi WHERE ipi_idproduto = $1";
    sqlx::query(q).bind(prodid).execute(pool).await?;
    Ok(())
}

// delete icms
#[allow(dead_code)]
pub async fn delete_icms(pool: &sqlx::PgPool, prodid: &i32) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_icms WHERE icms_idproduto = $1";
    sqlx::query(q).bind(prodid).execute(pool).await?;
    Ok(())
}

// delete icmsufdest
#[allow(dead_code)]
pub async fn delete_icmsufdest(pool: &sqlx::PgPool, prodid: &i32) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM nfe_icmsufdest WHERE icms_uf_idproduto = $1";
    sqlx::query(q).bind(prodid).execute(pool).await?;
    Ok(())
}

// delete produto
#[allow(dead_code)]
pub async fn delete_produto(
    pool: &sqlx::PgPool,
    nfeidnfe: &i32,
    idproduto: &i32,
) -> Result<(), Box<dyn Error>> {
    delete_cofins(pool, idproduto).await?;
    delete_pis(pool, idproduto).await?;
    delete_ipi(pool, idproduto).await?;
    delete_icms(pool, idproduto).await?;
    delete_icmsufdest(pool, idproduto).await?;
    let q = "DELETE FROM nfe_produto WHERE produto_idproduto = $1 AND produto_idnfe = $2";
    sqlx::query(q)
        .bind(idproduto)
        .bind(nfeidnfe)
        .execute(pool)
        .await?;

    Ok(())
}

// delete nfe
#[allow(dead_code)]
pub async fn delete_nfe(pool: &sqlx::PgPool, idnfe: &i32) -> Result<(), Box<dyn Error>> {
    let v = get_products_id_from_nfe(pool, idnfe).await?;
    for i in v {
        delete_produto(pool, idnfe, &i).await?;
    }
    let q = "DELETE FROM nfe WHERE nfe_idnfe = $1";
    let result = sqlx::query(q).bind(idnfe).execute(pool).await?;
    match result.rows_affected() {
        0 => Err("Nfe not found".into()),
        _ => Ok(()),
    }
}
