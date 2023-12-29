use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone)]
pub struct UserModel{
    pub firstname: String,
    pub secondname: String,
    pub email: String,
    pub uuid: Uuid,
    pub created_at: NaiveDateTime
}

#[derive(Debug,Clone)]
pub struct UserMicroModel{
    pub firstname: String,
    pub uuid: Uuid,
}

#[derive(Debug, Serialize,Deserialize)]
pub struct CreateUserModel{
    pub firstname: String,
    pub secondname: String,
    pub email: String,
    pub password: String 
}

#[derive(Serialize,Deserialize, FromRow)]
pub struct LoginUserModel{
    pub email: String,
    pub password: String 
}

#[derive(Serialize,Deserialize)]
pub struct LoginUserResponseModel{
    pub token: String
}

#[derive(Serialize,Deserialize)]
pub struct UpdateUserModel{
    pub firstname: String,
}