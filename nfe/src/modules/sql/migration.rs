use super::connection_postgres::start_connection;
#[allow(unused_imports)]
pub async fn main () {
    let pool = start_connection().await;
    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        println!("Error running migrations: {}", e);
    } else {
        println!("Migrations ran successfully");
    }
}