use crate::modules::json::structs::ender;

use super::super::json::structs::ender::Ender;

use super::connect::connect;
use sqlx::{pool, prelude::FromRow, Pool, Postgres, Row};
use std::error::Error;

#[tokio::main]
pub async fn get_idender(pool: &sqlx::PgPool, cep: i32, nro: i32) -> Result<(), Box<dyn Error>> {
    let q = "SELECT idender FROM ender WHERE cep = $1 AND nro = $2";
    let idender = sqlx::query(&q)
        .bind(cep)
        .bind(nro)
        .execute(pool)
        .await?;
    println!("{:?}", idender);
    Ok(())
}

pub async fn insert_ender(pool: &sqlx::PgPool, ender: &Ender) -> Result<(), Box<dyn Error>> {
    let q = "INSERT INTO ender (idender, cep, uf, cmun, cpais, nro, xbairro, xcpl, xlgr, xmun) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";
    let idender = sqlx::query(&q)
        .bind(2)
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
    Ok(())
}
