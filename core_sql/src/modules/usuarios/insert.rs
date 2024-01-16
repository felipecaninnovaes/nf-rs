use chrono::Utc;
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

use crate::structs::usuarios::struct_user::CreateUserModel;

#[allow(dead_code)]
pub async fn insert_user(pool: &PgPool, user: CreateUserModel) -> Result<(), Box<dyn Error>> {
    let uuid = Uuid::new_v4();
    let created_at = Utc::now().naive_utc().date(); // Change the type to NaiveDate
    
    let result = sqlx::query!(
        r#"INSERT INTO users (iduser, firstname, secondname, email, password, created_at) VALUES ($1, $2, $3, $4, $5, $6)"#,
        uuid,
        user.firstname,
        user.secondname,
        user.email,
        user.password,
        created_at
    )
    .execute(pool)
    .await?;
    

    match result.rows_affected() {
        1 => Ok(()),
        _ => Err("Failed to insert user".into()),
    }
}
