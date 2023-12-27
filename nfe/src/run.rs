use crate::modules::json::structs::nfe::Nfe;
use crate::modules::util::read_folder::listar_arquivos;
use dotenv::dotenv;
use crate::modules::sql::insert::{insert_nfe, insert_produto};
use crate::modules::sql::connection_postgres::start_connection;
// use std::error::Error;


#[tokio::main]
#[allow(dead_code)]
pub async fn database(){
    dotenv().ok();
    let _pool = start_connection().await;

    let values = listar_arquivos("nfe/tests/data").expect("Erro ao listar arquivos");

    for value in values {
        let input = Nfe::new(&value);
        println!("{:?}", input);
        let result = insert_nfe(&_pool, &input)
            .await
            .expect("Error inserting nfe");
        let _ = insert_produto(&_pool, &input.produtos, &result).await;
    }
}