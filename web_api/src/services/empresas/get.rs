use axum::{http::StatusCode, response::IntoResponse, Extension};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use super::struct_empresas::EmpresasGetModel;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn get_all_empresas(Extension(_pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let q = "SELECT * FROM empresas";
    match sqlx::query_as::<_, EmpresasGetModel>(q)
        .fetch_all(&_pool)
        .await
    {
        Ok(res) => {
            let json = serde_json::to_string(&res).expect("Error to convert json");
            (StatusCode::OK, json)
        }
        Err(_) => {
            let res = ErrorResponse {
                status: "error",
                message: "Error to get all empresas".to_string(),
            };
            let json = serde_json::to_string(&res).expect("Error to convert json");
            (StatusCode::INTERNAL_SERVER_ERROR, json)
        }
    }
}
