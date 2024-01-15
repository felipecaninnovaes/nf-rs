#[allow(unused_imports)]
use modules::usuarios::select::{select_user_from_email, select_user_from_id};

mod modules;
mod structs;


#[tokio::main]
async fn main() {
    let _pool = modules::connection::start_connection().await;
    let result = select_user_from_id(&_pool, "8a6fca50-6ae8-49a2-be2e-b369487f3aea").await;
    println!("{:?}", result)
}