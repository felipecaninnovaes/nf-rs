use chrono::Utc;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::structs::usuarios::struct_user::CreateUserModel;

#[allow(dead_code)]
pub async fn insert_user(pool: &PgPool, user: CreateUserModel) -> Result<(), Box<dyn Error>> {
    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc();
    let insert_user = format!(
        "INSERT INTO users (iduser, firstname, secondname, email, password, created_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
        uuid, user.firstname, user.secondname, user.email, user.password, created_at
    );
    match sqlx::query(&insert_user).execute(pool).await {
        Ok(_) => Ok(()),
        Err(_) => Err("Erro ao inserir usuario".into()),
    }
}
