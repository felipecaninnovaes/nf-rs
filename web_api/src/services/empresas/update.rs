use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use core_sql::modules::empresas::update::update_empresa;
use core_sql::modules::permissoes::select::select_all_user_permissions;
use core_sql::structs::empresas::empresa_struct::UpdateEmpresasModel;
use sqlx::{Pool, Postgres};

use crate::services::utils::parse::parse_uuid;

use crate::services::utils::{api_error::APIError, api_ok::APIOk};

pub async fn update_empresas(
    Extension(_pool): Extension<Pool<Postgres>>,
    path: Path<String>,
    Json(empresas): Json<UpdateEmpresasModel>,
) -> Result<impl IntoResponse, APIError> {
    let user_id = match parse_uuid(path.0) {
        Ok(uuid) => uuid,
        Err(_) => Err(APIError {
            message: "Invalid user id".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(41),
        })?,
    };
    let permissions = select_all_user_permissions(&_pool, user_id).await.unwrap();
    match permissions.iter().find(|&x| x.cnpj == empresas.cnpj) {
        Some(_) => {
            let result = update_empresa(&_pool, empresas).await;
            match result {
                Ok(_) => Ok(APIOk {
                    message: "Empresa atualizado com sucesso".to_owned(),
                    status_code: StatusCode::ACCEPTED,
                    data: None,
                }),
                Err(_) => Err(APIError {
                    message: "Erro ao atualizar empresa".to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(42),
                }),
            }
        }
        None => Err(APIError {
            message: "Você não tem permissão para atualizar essa empresa".to_owned(),
            status_code: StatusCode::FORBIDDEN,
            error_code: Some(43),
        }),
    }
}
