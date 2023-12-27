mod insert_in_database;
mod modules;
use dotenv::dotenv;
use insert_in_database::insert_in_database;

fn main() {
    dotenv().ok();
    let path = dotenv::var("NF_FOLDER_PATH").expect("Error reading path");
    insert_in_database(&path);
}