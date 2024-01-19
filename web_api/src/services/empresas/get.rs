use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use core_sql::{
    modules::{
        empresas::select::select_empresas_by_cnpj,
        permissoes::select::select_all_user_permissions,
    },
    structs::empresas::empresa_struct::EmpresasGetModel,
};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

// pub async fn get_empresas_from_cnpj(
//     pool: &sqlx::PgPool,
//     cnpj: String,
// ) -> Result<EmpresasGetModel, Box<dyn Error>> {
//     let q = "SELECT * FROM empresas WHERE cnpj = $1";
//     let mut empresas = select_empresas_by_cnpj(&pool, &cnpj).await?;
//     // for row in sqlx::query_as::<_, EmpresasGetModel>(q)
//     //     .bind(cnpj)
//     //     .fetch_all(pool)
//     //     .await?
//     // {
//     //     empresas.push(row);
//     // }
//     Ok(empresas)
// }

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
    match serde_json::to_string(&empresas) {
        Ok(json) => (StatusCode::OK, json),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error to convert json".to_owned(),
        ),
    }
}
