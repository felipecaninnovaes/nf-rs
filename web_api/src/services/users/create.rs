use crate::services::{auth::argon2::encrypt, utils::api_error::APIError};
use axum::{http::StatusCode, response::IntoResponse, Json, Extension};
use chrono::Utc;
use core_sql::modules::usuarios::select::select_user_from_email;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::struct_users::CreateUserModel;

pub async fn create_user(Extension(_pool): Extension<Pool<Postgres>>, Json(user): Json<CreateUserModel>) -> Result<impl IntoResponse, APIError> {

    let result = select_user_from_email(&_pool, &user.email).await.unwrap();

    if result.len() > 0 {
        return Err(APIError {
            message: "Usuario já cadastrado".to_owned(),
            status_code: StatusCode::CONFLICT,
            error_code: Some(41),
        });
    }

    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc();
    let encrypt_password = encrypt(user.password.as_str()).unwrap();
    let q = format!("INSERT INTO users (iduser, firstname, secondname, email, password, created_at) VALUES ('{}','{}', '{}', '{}', '{}','{}')" , uuid, user.firstname, user.secondname, user.email, encrypt_password, created_at);
    sqlx::query(&q).execute(&_pool).await.unwrap();
    Ok((StatusCode::OK, "Usuario criado com sucesso"))
}
