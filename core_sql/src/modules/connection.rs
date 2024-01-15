use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;

pub async fn start_connection() -> Pool<Postgres> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .expect("Error connecting to database")
}