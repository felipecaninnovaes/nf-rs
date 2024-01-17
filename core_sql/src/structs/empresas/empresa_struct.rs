use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct EmpresasModel {
    pub uuid: Uuid,
    pub nome: String,
    pub nome_fant: String,
    pub cnpj: String,
    pub rua: String,
    pub numero: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub cep: String,
    pub telefone: String,
    pub email: String,
    pub regime_tributario: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct EmpresasGetModel {
    pub idempresa: Uuid,
    pub nome: String,
    pub nome_fant: String,
    pub cnpj: String,
    pub rua: String,
    pub numero: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub cep: String,
    pub telefone: String,
    pub email: String,
    pub regime_tributario: String,
}


#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct EmpresasMicroModel {
    pub uuid: Uuid,
    pub nome: String,
    pub nome_fant: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct CreateEmpresasModel {
    pub nome: String,
    pub nome_fant: String,
    pub cnpj: String,
    pub rua: String,
    pub numero: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub cep: String,
    pub telefone: String,
    pub email: String,
    pub regime_tributario: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UpdateEmpresasModel {
    pub nome: String,
    pub nome_fant: String,
    pub cnpj: String,
    pub rua: String,
    pub numero: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub cep: String,
    pub telefone: String,
    pub email: String,
    pub regime_tributario: String,
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct PermissionsModel {
    pub permissions_idpermission: Uuid,
    pub permissions_user_id: Uuid,
    pub permissions_empresa_id: Uuid,
    pub permissions_allowed: bool,
}
