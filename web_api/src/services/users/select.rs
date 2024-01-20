use axum::{http::StatusCode, response::IntoResponse, Extension};
use sqlx::{Pool, Postgres};

use core_sql::modules::usuarios::select::select_all_users;

use crate::services::utils::{api_error::APIError, api_ok::APIOk};

pub async fn select_users(Extension(_pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let usuarios = select_all_users(&_pool).await.unwrap();
    match usuarios.len() {
        0 => Err(APIError {
            message: "Nenhum usuarios encontrado".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        }),
        _ => Ok(APIOk {
            message: "Usuarios encontrados com sucesso".to_owned(),
            status_code: StatusCode::OK,
            data: Some(serde_json::to_value(usuarios).unwrap()),
        }),
    }
}
