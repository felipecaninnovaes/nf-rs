use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UserModel {
    pub uuid: Uuid,
    pub firstname: String,
    pub secondname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UserMicroModel {
    pub firstname: String,
    pub uuid: Uuid,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct CreateUserModel {
    pub firstname: String,
    pub secondname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct LoginUserModel {
    pub email: String,
    pub password: String,
}


#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct LoginTockenCheckModel {
    pub email: String,
    pub firstname: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct LoginCheckModel {
    pub iduser: Uuid,
    pub email: String,
    pub firstname: String,
    pub password: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct LoginUserResponseModel {
    pub iduser: Uuid,
    pub firstname: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UpdateUserModel {
    pub firstname: String,
}
