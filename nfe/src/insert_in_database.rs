use crate::modules::json::structs::nfe::Nfe;
use crate::modules::sql::connection_postgres::start_connection;
use crate::modules::sql::insert::{insert_nfe, insert_produto};
use crate::modules::util::read_folder::{list_folder, remove_file};
use dotenv::dotenv;
// use std::error::Error;

#[allow(dead_code, unused_variables)]
pub async fn insert_in_database(path: &str) {
    dotenv().ok();
    let _pool = start_connection().await;

    let values = list_folder(path).expect("Error reading folder");

    for value in values {
        let input = Nfe::new(&value);
        let result = insert_nfe(&_pool, &input)
            .await
            .expect("Error inserting nfe");
        let _ = insert_produto(&_pool, &input.nfe_produtos, &result).await;
        remove_file(value.as_str()).expect("Error removing file");
    }
}
