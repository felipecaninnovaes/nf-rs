use axum::{http::StatusCode, Extension, Json};
use core_sql::modules::usuarios::select::select_user_by_email;
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::services::{utils::api_error::APIError, users::struct_users::{LoginUserModel, LoginUserResponseModel}};

use super::{
    jwt::encode_jwt,
     argon2::check,
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn login_user_post(
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(user_data): Json<LoginUserModel>,
) -> Result<Json<LoginUserResponseModel>, APIError> {
    let user = select_user_by_email(&_pool, &user_data.email).await.map_err(|_| APIError {
        message: "Failed to login".to_owned(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(41),
    })?;

    match check(&user_data.password, &user[0].password) {
        Ok(true) => (),
        _ => {
            return Err(APIError {
                message: "Failed to login".to_owned(),
                status_code: StatusCode::UNAUTHORIZED,
                error_code: Some(41),
            })
        }
    }

    let token = encode_jwt(user[0].email.clone(), user[0].iduser).map_err(|_| APIError {
        message: "Failed to login".to_owned(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(41),
    })?;

    Ok(Json(LoginUserResponseModel {
        iduser: user[0].iduser,
        firstname: user[0].firstname.clone(),
        email: user[0].email.clone(),
        token,
    }))
}
