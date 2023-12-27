use std::error::Error;

use super::select::get_products_id_from_nfe;

// delete ender
pub async fn delete_ender(
    pool: &sqlx::PgPool,
    nro: &String,
    cep: &String,
) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM ender WHERE nro = $1 AND cep = $2";
    sqlx::query_scalar(q).bind(nro).bind(cep).fetch_one(pool).await?;

    Ok(())
}

// delete emit
pub async fn delete_emit(
    pool: &sqlx::PgPool,
    cnpjcpf: &String,
    enderidender: &String,
) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM emit WHERE cnpjcpf = $1 AND enderidender = $2";
    sqlx::query_scalar(q).bind(cnpjcpf).bind(enderidender).fetch_one(pool).await?;

    Ok(())
}

// delete dest
pub async fn delete_dest(
    pool: &sqlx::PgPool,
    cnpjcpf: &String,
    enderidender: &String,
) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM dest WHERE cnpjcpf = $1 AND enderidender = $2";
    sqlx::query_scalar(q).bind(cnpjcpf).bind(enderidender).fetch_one(pool).await?;

    Ok(())
}

// delete pis
pub async fn delete_pis(
    pool: &sqlx::PgPool,
    prodid: &String,) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM pis WHERE produtoidproduto = $1";
    sqlx::query_scalar(q).bind(prodid).fetch_one(pool).await?;
    Ok(())
    }

// delete cofins
pub async fn delete_cofins(
    pool: &sqlx::PgPool,
    prodid: &String,) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM cofins WHERE produtoidproduto = $1";
    sqlx::query_scalar(q).bind(prodid).fetch_one(pool).await?;
    Ok(())
}

// delete ipi
pub async fn delete_ipi(
    pool: &sqlx::PgPool,
    prodid: &String,) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM ipi WHERE produtoidproduto = $1";
    sqlx::query_scalar(q).bind(prodid).fetch_one(pool).await?;
    Ok(())
}

// delete icms
pub async fn delete_icms(
    pool: &sqlx::PgPool,
    prodid: &String,) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM icms WHERE produtoidproduto = $1";
    sqlx::query_scalar(q).bind(prodid).fetch_one(pool).await?;
    Ok(())
}

// delete icmsufdest
pub async fn delete_icmsufdest(
    pool: &sqlx::PgPool,
    prodid: &String,) -> Result<(), Box<dyn Error>> {
    let q = "DELETE FROM icmsufdest WHERE produtoidproduto = $1";
    sqlx::query_scalar(q).bind(prodid).fetch_one(pool).await?;
    Ok(())
}

// delete produto
pub async fn delete_produto(
    pool: &sqlx::PgPool,
    nitem: &String,
    nfeidnfe: &String,
) -> Result<(), Box<dyn Error>> {
    delete_cofins(pool, nitem).await?;
    delete_pis(pool, nitem).await?;
    delete_ipi(pool, nitem).await?;
    delete_icms(pool, nitem).await?;
    delete_icmsufdest(pool, nitem).await?;
    let q = "DELETE FROM produto WHERE nitem = $1 AND nfeidnfe = $2";
    sqlx::query_scalar(q).bind(nitem).bind(nfeidnfe).fetch_one(pool).await?;

    Ok(())
}

// delete nfe
pub async fn delete_nfe(
    pool: &sqlx::PgPool,
    idnfe: &String,
) -> Result<(), Box<dyn Error>> {
    let v = get_products_id_from_nfe(pool, idnfe).await?;
    for i in v {
        delete_produto(pool, &i, idnfe).await?;
    }
    let q = "DELETE FROM nfe WHERE idnfe = $1";
    sqlx::query_scalar(q).bind(idnfe).fetch_one(pool).await?;

    Ok(())
}
