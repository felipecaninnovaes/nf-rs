use sqlx::{Row, prelude::FromRow, Postgres, Pool};
use std::error::Error;

#[tokio::main]
pub async fn connect() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let url = "postgres://admin:l11f06c10@postgres-development.homelab.felipecncloud.com/nfe";
    let pool = sqlx::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}