use sqlx::PgPool;
use std::error::Error;

use crate::structs::usuarios::struct_user::UserSelectModel;

#[allow(dead_code)]
pub async fn select_user_from_email(
    pool: &PgPool,
    email: &str,
) -> Result<Vec<UserSelectModel>, Box<dyn Error>> {
    let select_user = format!("SELECT * FROM users WHERE email = '{}'", email);
    match sqlx::query_as::<_, UserSelectModel>(&select_user)
        .fetch_all(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Usuario não encontrado".into()),
    }
}

#[allow(dead_code)]
pub async fn select_user_from_id(
    pool: &PgPool,
    iduser: &str,
) -> Result<Vec<UserSelectModel>, Box<dyn Error>> {
    let select_user = format!("SELECT * FROM users WHERE iduser = '{}'", iduser);
    match sqlx::query_as::<_, UserSelectModel>(&select_user)
        .fetch_all(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Usuario não encontrado".into()),
    }
}

#[allow(dead_code)]
pub async fn select_all_users(pool: &PgPool) -> Result<Vec<UserSelectModel>, Box<dyn Error>> {
    let select_user = "SELECT * FROM users";
    match sqlx::query_as::<_, UserSelectModel>(&select_user)
        .fetch_all(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Usuario não encontrado".into()),
    }
}