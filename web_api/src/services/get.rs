use axum::http::StatusCode;
use axum::response::IntoResponse;

use nfe::modules::sql::connection_postgres::start_connection;
use nfe::modules::sql::select::all_nfe;
// use nfe::modules::json::structs::nfe::NfeSelect;

pub async fn get_all_nfe() -> impl IntoResponse {
    let pool = start_connection().await;

    let json = all_nfe(&pool).await.unwrap();

    (StatusCode::OK, json)
}
