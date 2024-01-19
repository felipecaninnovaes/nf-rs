use axum::{
    debug_handler, extract::Path, http::StatusCode, response::IntoResponse, Extension, Json,
};
use core_sql::{
    modules::{
        empresas::{insert::insert_empresa, select::select_empresas_by_cnpj},
        permissoes::insert::insert_query,
    },
    structs::empresas::empresa_struct::CreateEmpresasModel,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::services::utils::{api_error::APIError, api_ok::APIOk};

#[debug_handler]
pub async fn create_empresas(
    path: Path<String>,
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(empresas): Json<CreateEmpresasModel>,
) -> Result<impl IntoResponse, APIError> {
    let result = select_empresas_by_cnpj(&_pool, &empresas.cnpj).await;
    let user_id = Uuid::parse_str(path.0.as_str()).unwrap();
    match result {
        Ok(_) => Err(APIError {
            message: "Empresa já cadastrada".to_owned(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(42),
        }),
        Err(_) => {
            let result = insert_empresa(&_pool, empresas).await;
            match result {
                Ok(uuid) => {
                    insert_query(&_pool, user_id, uuid, true).await.unwrap();
                    Ok(APIOk {
                        message: "Empresa cadastrado com sucesso".to_owned(),
                        status_code: StatusCode::CREATED,
                        data: None,
                    })
                }
                Err(_) => Err(APIError {
                    message: "Erro ao inserir empresa".to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(42),
                }),
            }
        }
    }
}
