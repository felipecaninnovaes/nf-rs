use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use nfe::modules::sql::select::get_permissions;
use sqlx::{Pool, Postgres};

use std::error::Error;

use crate::services::utils::parse::parse_uuid;

use super::struct_empresas::CreateEmpresasModel;

pub async fn update_empresas(
    Extension(_pool): Extension<Pool<Postgres>>,
    path: Path<String>,
    Json(empresas): Json<CreateEmpresasModel>,
) -> impl IntoResponse {
    let user_id = match parse_uuid(path.0) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::UNPROCESSABLE_ENTITY, "Invalid uuid".to_owned()),
    };
    let permissions = get_permissions(&_pool, user_id).await.unwrap();
    match permissions.iter().find(|&x| x.cnpj == empresas.cnpj) {
        Some(_) => {
            let q = format!("UPDATE empresas SET nome = '{}', nome_fant = '{}', cnpj = '{}', rua = '{}', numero = '{}', bairro = '{}', cidade = '{}', estado = '{}', cep = '{}', telefone = '{}', email = '{}', regime_tributario = '{}' WHERE cnpj = '{}'", empresas.nome.to_uppercase(), empresas.nome_fant.to_uppercase(), empresas.cnpj, empresas.rua.to_uppercase(), empresas.numero, empresas.bairro.to_uppercase(), empresas.cidade.to_uppercase(), empresas.estado.to_uppercase(), empresas.cep, empresas.telefone, empresas.email, empresas.regime_tributario.to_uppercase(), empresas.cnpj);
            sqlx::query(&q).execute(&_pool).await.unwrap();
            (StatusCode::OK, "Empresa atualizada com sucesso".to_owned())
        }
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                "You don't have permission to update this company".to_owned(),
            )
        }
    }
}
