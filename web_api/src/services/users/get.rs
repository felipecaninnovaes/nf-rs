use axum::{http::StatusCode, response::IntoResponse, Extension};
use sqlx::{Pool, Postgres};

use super::struct_users::UserListResponseModel;

pub async fn get_all_users(Extension(_pool): Extension<Pool<Postgres>>) -> impl IntoResponse {
    let q = "SELECT iduser, firstname, secondname, email FROM users";
    let result = sqlx::query_as::<_, UserListResponseModel>(q)
        .fetch_all(&_pool)
        .await
        .unwrap();
    match serde_json::to_string(&result) {
        Ok(json) => (StatusCode::OK, json),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error to convert json".to_owned()),
    }
}