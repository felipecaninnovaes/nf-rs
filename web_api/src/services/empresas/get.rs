use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use core_sql::{
    modules::{
        empresas::select::select_all_empresas_by_cnpj,
        permissoes::select::select_all_user_permissions,
    },
    structs::empresas::empresa_struct::EmpresasGetModel,
};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn get_empresas_from_cnpj(
    _pool: &sqlx::PgPool,
    cnpj: String,
) -> Result<Vec<EmpresasGetModel>, Box<dyn Error>> {
    let empresas = select_all_empresas_by_cnpj(_pool, cnpj.as_str()).await;
    match empresas {
        Ok(empresas) => Ok(empresas),
        Err(_) => Err("Empresa não encontrada".into()),
    }
}

pub async fn get_all_empresas(
    Extension(_pool): Extension<Pool<Postgres>>,
    path: Path<String>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(path.0.as_str()).unwrap();
    let vec_cnpjs = select_all_user_permissions(&_pool, user_id).await.unwrap();
    let mut empresas: Vec<EmpresasGetModel> = Vec::new();
    for row in vec_cnpjs {
        let res = get_empresas_from_cnpj(&_pool, row.cnpj).await.unwrap();
        for row in res {
            empresas.push(row);
        }
    }
    match serde_json::to_string(&empresas) {
        Ok(json) => (StatusCode::OK, json),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error to convert json".to_owned(),
        ),
    }
}
