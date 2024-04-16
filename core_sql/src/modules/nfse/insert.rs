#![allow(unused_imports, dead_code)]
use nfse::modules::structs::{Endereco, EnderecoID, Tomador, TomadorID};

use super::select::{select_nfse_endereco, select_nfse_tomador_id};

async fn insert_endereco(
    pool: &sqlx::PgPool,
    endereco: &Endereco,
) -> Result<EnderecoID, EnderecoID> {
    let endereco_id = select_nfse_endereco(pool, &endereco.numero, &endereco.cep).await;
    match endereco_id {
        Some(id) => Ok(id),
        None => {
            let result = sqlx::query!(
                r#"INSERT INTO nfse_endereco (logradouro, bairro, numero, complemento, cep, uf, codigo_municipio, codigo_pais) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"#,
                endereco.logradouro,
                endereco.bairro,
                endereco.numero,
                endereco.complemento,
                endereco.cep,
                endereco.uf,
                endereco.codigo_municipio,
                endereco.codigo_pais
            ).fetch_one(pool).await.unwrap();
            Ok(EnderecoID {
                endereco_id: result.id,
            })
        }
    }
}

async fn insert_tomador(pool: &sqlx::PgPool, tomador: &Tomador) -> Result<TomadorID, TomadorID> {
    let tomador_id = select_nfse_tomador_id(pool, &tomador.cnpj, &tomador.cpf).await;
    match tomador_id {
        Some(id) => Ok(TomadorID {
            tomador_id: id.tomador_id,
        }),
        None => {
            let result = sqlx::query!(
                r#"INSERT INTO nfse_tomador (cnpj, cpf, inscricao_municipal) VALUES ($1, $2, $3) RETURNING id"#,
                tomador.cnpj,
                tomador.cpf,
                tomador.inscricao_municipal
            ).fetch_one(pool).await.unwrap();
            Ok(TomadorID {
                tomador_id: result.id,
            })
        }
    }
}

// async fn insert_prestador(
//     pool: &sqlx::PgPool,
//     prestador: &Prestador,
// ) -> Result<PrestadorID, PrestadorID> {
//     let prestador_id = select_nfse_prestador_id(pool, &prestador.cnpj).await;
// }
