use crate::modules::json::structs::nfe::Nfe;
use dotenv::dotenv;
use crate::modules::sql::insert::{insert_nfe, insert_produto};
use crate::modules::sql::connection_postgres::start_connection;
use std::error::Error;

pub fn nfe() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    let input = Nfe::new(
        std::env::var("NF_FILE_PATH")
            .expect("NF_FILE_PATH must be set")
            .as_str(),
    );
    
    println!("{:?}", input);
    
    // let _ = database(input).await;
    
    Ok(())
}

#[tokio::main]
#[allow(dead_code)]
async fn database(input: Nfe) {
    let _pool = start_connection().await;

    let result = insert_nfe(&_pool, &input)
        .await
        .expect("Error inserting nfe");
    let _ = insert_produto(&_pool, &input.produtos, &result).await;
}