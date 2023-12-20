pub mod modules;

use std::error::Error; // Add missing import

use modules::sql::insert::{insert_nfe, insert_produto};

use crate::modules::json::structs::nfe::Nfe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input =
        Nfe::new("nfe/nf-xml-files-examples/nfe-pessoa-juridica.xml");


    let url = "postgres://admin:l11f06c10@postgres-development.homelab.felipecncloud.com/nfe";
    let pool = sqlx::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let result = insert_nfe(&pool, &input).await.expect("Error inserting nfe");
    let _ = insert_produto(&pool, &input.produtos, &result).await;
    // let produtos = &input.produtos.len();
    // println!("{}", produto.unwrap());
    Ok(())
}
