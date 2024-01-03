use nfe::{modules::sql::migration, insert_in_database::insert_in_database};
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    migration::main().await;
}