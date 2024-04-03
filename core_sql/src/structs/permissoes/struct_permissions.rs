use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permissions {
    pub permissions_user_id: Uuid,
    pub permissions_allowed: bool,
    pub cnpj: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PermissionsUpdate {
    pub permissions_idpermission: Uuid,
    pub permissions_user_id: Uuid,
    pub permissions_empresa_id: Uuid,
    pub permissions_allowed: bool,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct PermissionsModel {
    pub permissions_idpermission: Uuid,
    pub permissions_user_id: Uuid,
    pub permissions_empresa_id: Uuid,
    pub permissions_allowed: bool,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct PermissionsEmpresaId {
    pub permissions_empresa_id: Uuid,
}
