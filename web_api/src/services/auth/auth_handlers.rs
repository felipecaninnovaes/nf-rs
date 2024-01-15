use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::services::{utils::api_error::APIError, users::struct_users::{LoginUserModel, LoginUserResponseModel, LoginCheckModel}};

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
    let q = "SELECT iduser, email, firstname, password FROM users WHERE email = $1";
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

    let token = encode_jwt(user.email.clone(), user.iduser).map_err(|_| APIError {
        message: "Failed to login".to_owned(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(41),
    })?;

    Ok(Json(LoginUserResponseModel {
        iduser: user.iduser,
        firstname: user.firstname,
        email: user.email,
        token,
    }))
}
