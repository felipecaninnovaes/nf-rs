use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let check_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await;
    let pool = match check_pool {
        Ok(pool) => pool,
        Err(e) => panic!("Error connecting to database: {}", e),
    };

        let check_migrate = sqlx::migrate!("./migrations").run(&pool).await;

        match check_migrate {
            Ok(_) => println!("Migrations ran successfully"),
            Err(e) => println!("Error running migrations: {}", e),
        }
    pool
}