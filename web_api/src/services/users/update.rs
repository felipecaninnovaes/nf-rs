use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use core_sql::{
    modules::usuarios::update::update_user, structs::usuarios::struct_user::UserModelUpdate,
};
use sqlx::{Pool, Postgres};

use crate::services::utils::{api_ok::APIOk, api_error::APIError};

pub async fn update_user_post(
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(users): Json<UserModelUpdate>,
) -> Result<impl IntoResponse, APIError> {
    let user: UserModelUpdate = UserModelUpdate {
        iduser: users.iduser,
        firstname: users.firstname,
        secondname: users.secondname,
        email: users.email,
    };

    match update_user(&_pool, user).await {
        Ok(_) => Ok(APIOk {
            message: "Usuaio atualizado com sucesso".to_owned(),
            status_code: StatusCode::ACCEPTED,
        }),
        Err(_) => Err(APIError {
            message: "Erro ao inserir usuario".to_owned(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(42),
        }),
    }
}

