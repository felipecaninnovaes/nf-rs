use super::struct_users::UserModelUpdate;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::{Pool, Postgres};

pub async fn update_user(
    Extension(_pool): Extension<Pool<Postgres>>,
    Json(users): Json<UserModelUpdate>,
) -> impl IntoResponse {
    let q = format!(
        "UPDATE users SET firstname = '{}', secondname = '{}', email = '{}' WHERE iduser = '{}'",
        users.firstname, users.secondname, users.email, users.iduser
    );
    sqlx::query(&q).execute(&_pool).await.unwrap();
    (StatusCode::OK, "Usuario atualizado com sucesso".to_owned())
}
