use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::Utc;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::services::utils::api_error::APIError;

use super::struct_empresas::CreateEmpresasModel;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn create_empresas( Extension(_pool): Extension<Pool<Postgres>> ,Json(empresas): Json<CreateEmpresasModel>) -> Result<impl IntoResponse, APIError> {

    let select_user = format!("SELECT * FROM empresas WHERE cnpj = '{}'", empresas.cnpj);
    let result = sqlx::query(&select_user).fetch_one(&_pool).await;

    if result.is_ok() {
        return Err(APIError {
            message: "Empresa já cadastrado".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        });
    }

    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc();
    let q = format!("INSERT INTO empresas (idempresa, nome, nome_fant, cnpj, rua, numero, bairro, cidade, estado, cep, telefone, email, regime_tributario, created_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')" , uuid, empresas.nome, empresas.nome_fant, empresas.cnpj, empresas.rua, empresas.numero, empresas.bairro, empresas.cidade, empresas.estado, empresas.cep, empresas.telefone, empresas.email, empresas.regime_tributario, created_at);
    sqlx::query(&q).execute(&_pool).await.unwrap();
    Ok((StatusCode::OK, "Empresa criada com sucesso"))
}