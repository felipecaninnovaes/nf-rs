pub mod modules;

use std::error::Error; // Add missing import

use modules::sql::{connect::connect, insert::get_idender};
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
        Nfe::new("nfe/nf-xml-files-examples/_NFe33231219300038000108550010026273031152863530.xml");
    // println!("{:?}", input);


    let url = "postgres://admin:l11f06c10@postgres-development.homelab.felipecncloud.com/nfe";
    let pool = sqlx::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // insert_ender(&pool, &input.emit.ender_emit).await?;
    get_idender(&pool, 20910180, 486);
    Ok(())
}

// async fn insert_ender_main(pool: &sqlx::PgPool, ender: &Ender) -> Result<(), Box<dyn Error>> {
//     let q = "INSERT INTO ender (idender, cep, uf, cmun, cpais, nro, xbairro, xcpl, xlgr, xmun) VALUES ('1', '2', '3', '4', '5', '6', '7', '8', '9', '10')";
//     let idender = sqlx::query("INSERT INTO ender (idender, cep, uf, cmun, cpais, nro, xbairro, xcpl, xlgr, xmun) VALUES ('1', '2', '3', '4', '5', '6', '7', '8', '9', '10')")
//         .execute(pool) // Remove the extra reference
//         .await?;
//     Ok(())
// }
