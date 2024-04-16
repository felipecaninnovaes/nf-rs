use sqlx::PgPool;
use std::error::Error;

use crate::{
    modules::usuarios::select::select_user_by_id, structs::usuarios::struct_user::UserModelUpdate,
};

#[allow(dead_code)]
pub async fn update_user(pool: &PgPool, user: UserModelUpdate) -> Result<(), Box<dyn Error>> {
    let user_exists = select_user_by_id(pool, &user.iduser.to_string()).await?;
    if user_exists.is_empty() {
        return Err("User not found".into());
    }
    sqlx::query!(
        r#"UPDATE users SET firstname = $1, secondname = $2, email = $3 WHERE id = $4"#,
        user.firstname,
        user.secondname,
        user.email,
        user.iduser,
    )
    .execute(pool)
    .await?;
    Ok(())
}
