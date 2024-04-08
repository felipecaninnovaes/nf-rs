#![allow(unused_imports, dead_code, unused_variables)]
use std::error::Error;

use nfse::structs::{Endereco, EnderecoId, Nfse, NfseId};

use super::select::select_endereco_id_from_cep_numero;

// pub async fn insert_nfse(pool: &sqlx::PgPool, nfse: Nfse) -> Result<(), Box<dyn Error>> {
//     let prestador_enderco: &Endereco = &Endereco {
//         logradouro: nfse.prestador.endereco,
//         bairro: nfse.prestador.bairro,
//         numero: nfse.prestador.numero,
//         complemento: nfse.prestador.complemento,
//         cep: nfse.prestador.cep,
//         uf: nfse.prestador.uf,
//         codigo_municipio: nfse.prestador.codigo_municipio,
//         codigo_pais: nfse.prestador.codigo_pais,
//     };
//     let endereco_id = insert_endereco_sql(pool, &prestador_enderco).await;
//     let prestador_id =
//         insert_prestador_sql(pool, &nfse.prestador, endereco_id.unwrap().endereco_id).await;
//     Ok(())
// }

// endereco: logradouro bairro numero complemento cep uf codigo_municipio codigo_pais
async fn insert_endereco_sql(
    pool: &sqlx::PgPool,
    endereco: &nfse::structs::Endereco,
) -> Result<EnderecoId, EnderecoId> {
    match select_endereco_id_from_cep_numero(pool, &endereco.cep, &endereco.numero).await {
        Ok(result) => Ok(result),
        Err(_) => {
            let result = sqlx::query!(
                r#"INSERT INTO endereco (logradouro, bairro, numero, complemento, cep, uf, codigo_municipio, codigo_pais)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"#,
                endereco.logradouro,
                endereco.bairro,
                endereco.numero,
                endereco.complemento,
                endereco.cep,
                endereco.uf,
                endereco.codigo_municipio,
                endereco.codigo_pais,
            )
            .fetch_one(pool)
            .await
            .unwrap();
            Ok(EnderecoId {
                endereco_id: result.id,
            })
        }
    }
}

// prestador: cnpj inscricao_municipal nome_fantasia razao_social email telefone endereco_id
async fn insert_prestador_sql(
    pool: &sqlx::PgPool,
    prestador: &nfse::structs::Prestador,
    endereco_id: i32,
) -> Result<i32, i32> {
    let result = sqlx::query!(
        r#"INSERT INTO prestador (cnpj, inscricao_municipal, nome_fantasia, razao_social, email, telefone, endereco_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id"#,
        prestador.cnpj,
        prestador.inscricao_municipal,
        prestador.nome_fantasia,
        prestador.razao_social,
        prestador.email,
        prestador.telefone,
        endereco_id,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    Ok(result.id)
}
