use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::utils;

#[derive(Serialize, Deserialize)]
pub struct Cliams {
    pub id: Uuid,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn encode_jwt(email: String, id: Uuid) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claim = Cliams {
        id: id,
        email,
        iat: now.timestamp() as usize,
        exp: (now + expire).timestamp() as usize,
    };
    let secret = (*utils::constants::TOKEN).clone();

    return encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Cliams>, StatusCode> {
    let secret = (*utils::constants::TOKEN).clone();
    let res: Result<TokenData<Cliams>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    res
}
