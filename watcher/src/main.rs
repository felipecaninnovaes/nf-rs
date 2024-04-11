use core_handler_nf::insert_in_database_handler::insert_in_database_handler;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let path = format!("{}/nfe", dotenv::var("UPLOAD_PATH").unwrap());
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        insert_in_database_handler(&path).await;
    }
}
