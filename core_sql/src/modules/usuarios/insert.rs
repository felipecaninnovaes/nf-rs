use sqlx::PgPool;
use std::error::Error;

use crate::{
    modules::utils::create::{create_id_and_current_date, CreateIdAndCurrentDateModel},
    structs::usuarios::struct_user::CreateUserModel,
};

#[allow(dead_code)]
pub async fn insert_user(pool: &PgPool, user: CreateUserModel) -> Result<(), Box<dyn Error>> {
    let id_and_current_date: CreateIdAndCurrentDateModel = create_id_and_current_date();

    let result = sqlx::query!(
        r#"INSERT INTO users (iduser, firstname, secondname, email, password, created_at) VALUES ($1, $2, $3, $4, $5, $6)"#,
        id_and_current_date.id,
        user.firstname,
        user.secondname,
        user.email,
        user.password,
        id_and_current_date.current_date,
    )
    .execute(pool)
    .await?;

    match result.rows_affected() {
        1 => Ok(()),
        _ => Err("Failed to insert user".into()),
    }
}
