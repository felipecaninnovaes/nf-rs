use crate::services::{auth::argon2::encrypt, utils::api_error::APIError};
use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use dotenv::dotenv;
use nfe::modules::sql::connection_postgres::start_connection;
use uuid::Uuid;

use super::struct_users::CreateUserModel;

pub async fn create_user(Json(user): Json<CreateUserModel>) -> Result<impl IntoResponse, APIError> {
    dotenv().ok();

    let _pool = start_connection().await;

    let select_user = format!("SELECT * FROM users WHERE email = '{}'", user.email);
    let result = sqlx::query(&select_user).fetch_one(&_pool).await;

    if result.is_ok() {
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
