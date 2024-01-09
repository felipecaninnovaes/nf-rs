use axum::{http::StatusCode, response::IntoResponse, Extension, extract::Path};
use nfe::modules::sql::select::get_permissions;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::struct_empresas::EmpresasGetModel;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn get_all_empresas(Extension(_pool): Extension<Pool<Postgres>>, path: Path<String>) -> impl IntoResponse {
    let user_id = Uuid::parse_str(path.0.as_str()).unwrap();
    let vec_cnpjs = get_permissions(&_pool, user_id).await.unwrap();
    let mut empresas: Vec<EmpresasGetModel> = Vec::new();
    let q = "SELECT * FROM empresas WHERE cnpj = $1";
    for row in vec_cnpjs {
        let res = sqlx::query_as::<_, EmpresasGetModel>(q)
            .bind(row.cnpj)
            .fetch_all(&_pool)
            .await
            .unwrap();
        for row in res {
            empresas.push(row);
        }
    }
    match serde_json::to_string(&empresas) {
        Ok(json) => (StatusCode::OK, json),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error to convert json".to_owned()),
    }    
}
