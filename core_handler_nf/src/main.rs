use dotenv::dotenv;
use nfe::modules::sql::migration;
#[tokio::main]
async fn main() {
    dotenv().ok();
    migration::main().await;
}


