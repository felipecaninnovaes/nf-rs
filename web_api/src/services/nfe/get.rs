use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use nfe::modules::sql::connection_postgres::start_connection;
use nfe::modules::sql::select::{all_nfe, nfe_by_dest, nfe_by_emit};
// use nfe::modules::json::structs::nfe::NfeSelect;

// get all nfe
pub async fn get_all_nfe() -> impl IntoResponse {
    let pool = start_connection().await;

    let result = all_nfe(&pool).await.unwrap();

    (StatusCode::OK, result)
}

// get nfe by emit
pub async fn get_nfe_by_emit(path: Path<i32>) -> impl IntoResponse {
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