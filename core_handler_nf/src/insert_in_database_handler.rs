use core_sql::modules::{
    connection::start_connection,
    nfe::{nfe_sql::insert::insert_nfe_sql, produtos::insert::insert_produto_sql},
};
use dotenv::dotenv;
use nfe::modules::{json::structs::nfe_struct::Nfe, util::read_folder::list_folder};

#[allow(dead_code, unused_variables)]
pub async fn insert_in_database_handler(path: &str) {
    dotenv().ok();
    let _pool = start_connection().await;

    let values = list_folder(path).expect("Error reading folder");

    for value in values {
        let input = Nfe::new(&value);
        let result = insert_nfe_sql(&_pool, &input)
            .await
            .expect("Error inserting nfe");
        let _ = insert_produto_sql(&_pool, &input.nfe_produtos, &result.nfe_idnfe).await;
    }
}