use axum::{extract::Path, response::IntoResponse, Extension, http::StatusCode};
use nfe::modules::sql::delete::delete_nfe;
use sqlx::{Pool, Postgres};

pub async fn delete_nfe_by_id(
    Extension(_pool): Extension<Pool<Postgres>>,
    path: Path<i32>,
) -> impl IntoResponse {
    let result = delete_nfe(&_pool, &path.0).await;
    match result {
        Ok(_) =>  (StatusCode::OK, "Nfe deleted"),
        Err(_) => (StatusCode::NOT_FOUND, "Nfe not found"),
    }
}
