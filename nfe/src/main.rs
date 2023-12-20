pub mod modules;

use std::error::Error; // Add missing import

use modules::sql::insert::get_idender;

// use modules::sql::{connect::connect, insert::get_idender};
use crate::modules::{json::structs::nfe::Nfe, sql::insert::insert_ender};

struct Ender {
    idender: String,
    cep: String,
    uf: String,
    cmun: String,
    cpais: String,
    nro: String,
    xbairro: String,
    xcpl: String,
    xlgr: String,
    xmun: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> { // Update return type
    let input =
        Nfe::new("nfe/nf-xml-files-examples/nfe-pessoa-fisica.xml");
    // println!("{:?}", input);


    let url = "postgres://admin:l11f06c10@postgres-development.homelab.felipecncloud.com/nfe";
    let pool = sqlx::PgPool::connect(url).await?;

    // sqlx::migrate!("./migrations").run(&pool).await?;

        let result = insert_ender(&pool, input.emit.ender_emit).await?;

    // let result = get_idender(&pool, input.emit.ender_emit.nro, input.emit.ender_emit.cep).await?;
    println!("{:?}", result);
    Ok(())
}
