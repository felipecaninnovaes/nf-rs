use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permissions {
    pub permissions_user_id: Uuid,
    pub permissions_allowed: bool,
    pub cnpj: String,
}
