use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use core_sql::modules::connection::start_connection;
use core_sql::modules::nfe::nfe_sql::select::{all_nfe, nfe_by_dest, nfe_by_emit};
use uuid::Uuid;

use crate::services::utils::api_error::APIError;
use crate::services::utils::api_ok::APIOk;

// get all nfe
pub async fn get_all_nfe(path: Path<String>) -> impl IntoResponse {
    let pool = start_connection().await;

    let user_id = Uuid::parse_str(path.0.as_str()).unwrap();

    let result = all_nfe(&pool, user_id).await.unwrap();
    match result.len() {
        2 => Err(APIError {
            message: "Nfe not found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        }),
        _ => Ok(APIOk {
            message: "Nfe found".to_owned(),
            status_code: StatusCode::OK,
            data: Some(serde_json::from_str(result.as_str()).unwrap()),
        }),
    }
}

// get nfe by emit
pub async fn get_nfe_by_emit(path: Path<String>) -> impl IntoResponse {
    println!("{}", path.0);
    let pool = start_connection().await;

    let json = nfe_by_emit(&pool, &path).await.unwrap();

    match json.len() {
        2 => Err(APIError {
            message: "Nfe not found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        }),
        _ => Ok(APIOk {
            message: "Nfe found".to_owned(),
            status_code: StatusCode::OK,
            data: Some(serde_json::from_str(json.as_str()).unwrap()),
        }),
    }
}

// get nfe by dest
pub async fn get_nfe_by_dest(path: Path<i32>) -> impl IntoResponse {
    let pool = start_connection().await;

    let json = nfe_by_dest(&pool, &path).await.unwrap();

    match json.len() {
        2 => Err(APIError {
            message: "Nfe not found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        }),
        _ => Ok(APIOk {
            message: "Nfe found".to_owned(),
            status_code: StatusCode::OK,
            data: Some(serde_json::from_str(json.as_str()).unwrap()),
        }),
    }
}
