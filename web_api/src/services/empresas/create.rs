use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::Utc;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::services::{empresas::struct_empresas::PermissionsModel, utils::api_error::APIError};

use super::struct_empresas::{CreateEmpresasModel, EmpresasGetModel};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn create_empresas(
    path: Path<String>,
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(empresas): Json<CreateEmpresasModel>,
) -> Result<impl IntoResponse, APIError> {
    let select_empresa = format!("SELECT * FROM empresas WHERE cnpj = '{}'", empresas.cnpj);
    let result = sqlx::query_as::<_, EmpresasGetModel>(&select_empresa)
        .fetch_one(&_pool)
        .await;
    let user_id = Uuid::parse_str(path.0.as_str()).unwrap();
    if result.is_ok() {
        create_permissions(&_pool, user_id, result.expect("erro").idempresa, true).await;
        return Err(APIError {
            message: "Empresa já cadastrado".to_owned(),
            status_code: StatusCode::CONFLICT,
            error_code: Some(41),
        });
    }

    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc();
    let q = format!("INSERT INTO empresas (idempresa, nome, nome_fant, cnpj, rua, numero, bairro, cidade, estado, cep, telefone, email, regime_tributario, created_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')" , uuid, empresas.nome.to_uppercase(), empresas.nome_fant.to_uppercase(), empresas.cnpj, empresas.rua.to_uppercase(), empresas.numero, empresas.bairro.to_uppercase(), empresas.cidade.to_uppercase(), empresas.estado.to_uppercase(), empresas.cep, empresas.telefone, empresas.email, empresas.regime_tributario.to_uppercase(), created_at);
    sqlx::query(&q).execute(&_pool).await.unwrap();
    create_permissions(&_pool, user_id, uuid, true).await;

    Ok((StatusCode::CREATED, "Empresa criada com sucesso"))
}

pub async fn create_permissions(
    pool: &Pool<Postgres>,
    user_id: Uuid,
    empresa_id: Uuid,
    allowed: bool,
) {
    let uuid = Uuid::new_v4();
    let select_permissions = format!("SELECT * FROM permissions WHERE permissions_user_id = '{}' AND permissions_empresa_id = '{}'", user_id, empresa_id);
    let _result = sqlx::query_as::<_, PermissionsModel>(&select_permissions)
        .fetch_all(pool)
        .await;

    if let Ok(result) = _result {
        if result.is_empty() {
            let created_at = Utc::now().naive_utc();
            let q = format!("INSERT INTO permissions (permissions_idpermission, permissions_user_id, permissions_empresa_id, permissions_allowed, permissions_created_at) VALUES ('{}', '{}', '{}', '{}', '{}')" , uuid, user_id, empresa_id, allowed, created_at);
            sqlx::query(&q).execute(pool).await.unwrap();
        }
    }
}
