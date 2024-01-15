use axum::{http::StatusCode, response::IntoResponse, Extension};
use sqlx::{Pool, Postgres};

use core_sql::modules::usuarios::select::select_all_users;

pub async fn select_users(Extension(_pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let result = select_all_users(&_pool).await.unwrap();
    match serde_json::to_string(&result) {
        Ok(json) => (StatusCode::OK, json),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error to convert json".to_owned(),
        ),
    }
}
