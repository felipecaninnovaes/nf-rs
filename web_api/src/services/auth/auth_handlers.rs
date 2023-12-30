use axum::{Json, http::StatusCode, response::IntoResponse, Extension};
use chrono::Utc;
use dotenv::dotenv;
use nfe::modules::sql::connection_postgres::start_connection;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use serde::Serialize;

use crate::services::utils::api_error::APIError;

use super::{struct_user::{CreateUserModel, LoginUserModel, LoginUserResponseModel, LoginTockenCheckModel}, jwt::encode_jwt};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn create_user(Json(user): Json<CreateUserModel>) -> impl IntoResponse {
    dotenv().ok();

    let _pool = start_connection().await;

    let select_user = format!("SELECT * FROM users WHERE email = '{}'", user.email);
    sqlx::query(&select_user).fetch_one(&_pool).await.unwrap();

    if !select_user.is_empty() {
        return (StatusCode::BAD_REQUEST, "User already exists");
    }

    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc();
    let q = format!("INSERT INTO users (iduser, firstname, secondname, email, password, created_at) VALUES ('{}','{}', '{}', '{}', '{}','{}')" , uuid, user.firstname, user.secondname, user.email, user.password, created_at);
    sqlx::query(&q).execute(&_pool).await.unwrap();
    (StatusCode::OK, "User created")
}

pub async fn login_user_post(
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(user_data): Json<LoginUserModel>
) -> Result<Json<LoginUserResponseModel>,APIError> {

    let q = "SELECT email, firstname FROM users WHERE email = $1 AND password = $2";
    let user = sqlx::query_as::<_, LoginTockenCheckModel>(q).bind(user_data.email).bind(user_data.password).fetch_one(&_pool).await.map_err(|_| APIError { message: "Failed to login".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41) })?;

    let token = encode_jwt(user.email.clone())
    .map_err(|_| APIError { message: "Failed to login".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41) })?;

    Ok(Json(LoginUserResponseModel {firstname: user.firstname, email: user.email, token}))

}