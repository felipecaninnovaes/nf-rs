pub mod modules;

use std::error::Error; // Add missing import

use crate::modules::json::structs::nfe::Nfe;
use dotenv::dotenv;
use modules::sql::connection_postgres::start_connection;
use modules::sql::insert::{insert_nfe, insert_produto};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let input =
        Nfe::new("nfe/nf-xml-files-examples/nfe-pessoa-juridica.xml");
    let _pool = start_connection().await;

    let result = insert_nfe(&_pool, &input)
        .await
        .expect("Error inserting nfe");
    let _ = insert_produto(&_pool, &input.produtos, &result).await;
    Ok(())
}
