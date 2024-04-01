use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use core_sql::modules::nfe::nfe_sql::delete::delete_nfe;
use sqlx::{Pool, Postgres};

use crate::services::utils::{api_error::APIError, api_ok::APIOk};

pub async fn delete_nfe_by_id(
    Extension(_pool): Extension<Pool<Postgres>>,
    path: Path<i32>,
) -> impl IntoResponse {
    let result = delete_nfe(&_pool, &path.0).await;
    match result {
        Ok(_) => Ok(APIOk {
            message: "Nfe deleted successfully".to_owned(),
            status_code: StatusCode::OK,
            data: None,
        }),
        Err(_) => Err(APIError {
            message: "Error deleting nfe".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(44),
        }),
    }
}
