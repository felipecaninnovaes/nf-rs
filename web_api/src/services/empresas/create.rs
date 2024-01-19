use axum::{
    debug_handler, extract::Path, http::StatusCode, response::IntoResponse, Extension, Json,
};
use chrono::Utc;
use core_sql::modules::permissoes::{
    insert::insert_permission, select::select_user_permission_by_empresa_id,
};
use sqlx::{Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

use crate::services::utils::api_error::APIError;

use super::struct_empresas::{CreateEmpresasModel, EmpresasGetModel};
#[debug_handler]
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
        let _result: Result<(), Box<dyn Error + Sync + Send>> =
            insert_permission(&_pool, user_id, uuid, true).await;
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
    let _result: Result<(), Box<dyn Error + Sync + Send>> =
        insert_permission(&_pool, user_id, uuid, true).await;
    // let _result = create_permissions(&_pool, user_id, uuid, true).await;

    Ok((StatusCode::CREATED, "Empresa criada com sucesso"))
}

// pub async fn create_permissions(
//     pool: &Pool<Postgres>,
//     user_id: Uuid,
//     empresa_id: Uuid,
//     allowed: bool,
// ) {
//     let _result = select_user_permission_by_empresa_id(pool, user_id, empresa_id).await;
//     match _result {
//         Ok(_) => {}
//         Err(_) => {
//             let _result = insert_permission(pool, user_id, empresa_id, allowed).await;
//             if _result.is_ok() {}
//         }
//     }
// }
