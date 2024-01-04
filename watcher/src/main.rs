use dotenv::dotenv;
use nfe::insert_in_database::insert_in_database;



#[tokio::main]
async fn main() {
    dotenv().ok();
    let path = format!("{}/nfe", dotenv::var("UPLOAD_PATH").unwrap());
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
        insert_in_database(&path).await;
    }
}
