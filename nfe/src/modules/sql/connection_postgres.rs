use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .expect("Error connecting to database");

    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        println!("Error running migrations: {}", e);
    } else {
        println!("Migrations ran successfully");
    }

    pool
}