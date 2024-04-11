use core_sql::modules::{
    connection::start_connection,
    nfe::{nfe_sql::insert::insert_nfe_sql, produtos::insert::insert_produto_sql},
};
use dotenv::dotenv;
use nfe::modules::json::structs::nfe_struct::Nfe;
use utils::core::fs::Dir;

#[allow(dead_code, unused_variables)]
pub async fn insert_in_database_handler(path: &str) {
    dotenv().ok();
    let _pool = start_connection().await;

    let values = Dir::read_dir(path, ".xml");

    for value in values.dir_files.expect("Error reading folder") {
        let input = Nfe::new(&value);
        let result = insert_nfe_sql(&_pool, &input)
            .await
            .expect("Error inserting nfe");
        let _ = insert_produto_sql(&_pool, &input.nfe_produtos, &result.nfe_idnfe).await;
    }
}
