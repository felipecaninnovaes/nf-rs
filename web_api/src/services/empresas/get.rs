use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use core_sql::{
    modules::{
        empresas::select::select_empresas_by_cnpj, permissoes::select::select_all_user_permissions,
    },
    structs::empresas::empresa_struct::EmpresasGetModel,
};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::services::utils::{api_error::APIError, api_ok::APIOk};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn get_all_empresas(
    Extension(_pool): Extension<Pool<Postgres>>,
    path: Path<String>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(path.0.as_str()).unwrap();
    let vec_cnpjs = select_all_user_permissions(&_pool, user_id).await.unwrap();
    let mut empresas: Vec<EmpresasGetModel> = Vec::new();
    for row in vec_cnpjs {
        let res = select_empresas_by_cnpj(&_pool, &row.cnpj).await.unwrap();
        empresas.push(res);
    }
    match empresas.len() {
        0 => Err(APIError {
            message: "Nenhuma empresa encontrada".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        }),
        _ => Ok(APIOk {
            message: "Empresas encontradas com sucesso".to_owned(),
            status_code: StatusCode::OK,
            data: Some(serde_json::to_value(empresas).unwrap()),
        }),
    }
}
