use sqlx::PgPool;
use std::error::Error;

use crate::structs::usuarios::struct_user::UserModelUpdate;

#[allow(dead_code)]
pub async fn update_user(pool: &PgPool, user: UserModelUpdate) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        r#"UPDATE users SET firstname = $1, secondname = $2, email = $3 WHERE iduser = $4"#,
        user.firstname,
        user.secondname,
        user.email,
        user.iduser,
    )
    .execute(pool)
    .await?;

    Ok(())
}
