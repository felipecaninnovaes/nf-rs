use std::future::IntoFuture;

use super::super::json::structs::ender::Ender;



use std::error::Error;
use sqlx::{Error as SqlxError, Row};

// read enderid from database
pub async fn get_idender(pool: &sqlx::PgPool, nro: &String, cep: &String) -> Result<i32, Box<dyn Error>> {
    let q = "SELECT idender FROM ender WHERE nro = $1 AND cep = $2";
    let idender = sqlx::query(&q)
        .bind(nro)
        .bind(cep)
        .fetch_one(pool).await?.get::<i32, _>(0);
    Ok(idender)
}


pub async fn insert_ender(pool: &sqlx::PgPool, ender: Ender) -> Result<i32, Box<dyn Error>> {
    let q = "INSERT INTO ender (idender, cep, uf, cmun, cpais, nro, xbairro, xcpl, xlgr, xmun) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING idender";
    sqlx::query(&q)
        .bind(3)
        .bind(&ender.cep)
        .bind(&ender.uf)
        .bind(&ender.c_mun)
        .bind(&ender.c_pais)
        .bind(&ender.nro)
        .bind(&ender.x_bairro)
        .bind(&ender.x_cpl)
        .bind(&ender.x_lgr)
        .bind(&ender.x_mun)
        .execute(pool) // Remove the extra reference
        .await?;

    let Result = get_idender(pool, &ender.nro, &ender.cep).await?;
    Ok(Result)
}
