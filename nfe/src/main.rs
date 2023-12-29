use nfe::modules::sql::migration;
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    migration::main().await;
}