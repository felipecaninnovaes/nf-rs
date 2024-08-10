use dotenv::dotenv;
pub mod modules;
use modules::sql::migration;
#[tokio::main]
async fn main() {
    dotenv().ok();
    migration::main().await;
}
