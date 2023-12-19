use std::error::Error;
use sqlx::Connection;
use sqlx::Row;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://admin:l11f06c10@10.15.1.27:5432/postgres";
    let mut conn = sqlx::PgConnection::connect(url).await?;
    Ok(()) 

} 