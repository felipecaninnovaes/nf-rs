use crate::services::{
    auth::argon2::encrypt,
    utils::{api_error::APIError, api_ok::APIOk},
};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use core_sql::{
    modules::usuarios::{insert::insert_user, select::select_user_by_email},
    structs::usuarios::struct_user::CreateUserModel,
};
use sqlx::{Pool, Postgres};

pub async fn create_user(
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(user): Json<CreateUserModel>,
) -> Result<impl IntoResponse, APIError> {
    let result = select_user_by_email(&_pool, &user.email).await.unwrap();

    if !result.is_empty() {
        return Err(APIError {
            message: "Usuario já cadastrado".to_owned(),
            status_code: StatusCode::CONFLICT,
            error_code: Some(41),
        });
    }

    let encrypt_password = encrypt(user.password.as_str()).unwrap();
    let user = CreateUserModel {
        firstname: user.firstname,
        secondname: user.secondname,
        email: user.email,
        password: encrypt_password,
    };
    match insert_user(&_pool, user).await {
        Ok(_) => Ok(APIOk {
            message: "Usuario cadastrado com sucesso".to_owned(),
            status_code: StatusCode::CREATED,
            data: None,
        }),
        Err(_) => Err(APIError {
            message: "Erro ao inserir usuario".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(42),
        }),
    }
}
