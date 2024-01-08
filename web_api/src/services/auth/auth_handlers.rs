use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::Utc;
use dotenv::dotenv;
use nfe::modules::sql::connection_postgres::start_connection;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::services::utils::api_error::APIError;

use super::{
    jwt::encode_jwt,
    struct_user::{CreateUserModel, LoginUserModel, LoginUserResponseModel, LoginCheckModel}, argon2::{encrypt, check},
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn create_user(Json(user): Json<CreateUserModel>) -> Result<impl IntoResponse, APIError> {
    dotenv().ok();

    let _pool = start_connection().await;

    let select_user = format!("SELECT * FROM users WHERE email = '{}'", user.email);
    let result = sqlx::query(&select_user).fetch_one(&_pool).await;

    if result.is_ok() {
        return Err(APIError {
            message: "Usuario já cadastrado".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
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

pub async fn login_user_post(
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(user_data): Json<LoginUserModel>,
) -> Result<Json<LoginUserResponseModel>, APIError> {
    let q = "SELECT email, firstname, password FROM users WHERE email = $1";
    let user = sqlx::query_as::<_, LoginCheckModel>(q)
        .bind(user_data.email)
        .fetch_one(&_pool)
        .await
        .map_err(|_| APIError {
            message: "Failed to login".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
            error_code: Some(41),
        })?;

    match check(&user_data.password, &user.password) {
        Ok(true) => (),
        _ => {
            return Err(APIError {
                message: "Failed to login".to_owned(),
                status_code: StatusCode::UNAUTHORIZED,
                error_code: Some(41),
            })
        }
    }

    let token = encode_jwt(user.email.clone()).map_err(|_| APIError {
        message: "Failed to login".to_owned(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(41),
    })?;

    Ok(Json(LoginUserResponseModel {
        firstname: user.firstname,
        email: user.email,
        token,
    }))
}
