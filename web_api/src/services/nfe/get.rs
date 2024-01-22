use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use core_sql::modules::connection::start_connection;
use core_sql::modules::nfe::nfe_sql::select::{all_nfe, nfe_by_dest, nfe_by_emit};
use uuid::Uuid;

// get all nfe
pub async fn get_all_nfe(path: Path<String>) -> impl IntoResponse {
    let pool = start_connection().await;

    let user_id = Uuid::parse_str(path.0.as_str()).unwrap();

    let result = all_nfe(&pool, user_id).await.unwrap();

    (StatusCode::OK, result)
}

// get nfe by emit
pub async fn get_nfe_by_emit(path: Path<String>) -> impl IntoResponse {
    println!("{}", path.0);
    let pool = start_connection().await;

    let json = nfe_by_emit(&pool, &path).await.unwrap();

    (StatusCode::OK, json)
}

// get nfe by dest
pub async fn get_nfe_by_dest(path: Path<i32>) -> impl IntoResponse {
    let pool = start_connection().await;

    let json = nfe_by_dest(&pool, &path).await.unwrap();

    (StatusCode::OK, json)
}
