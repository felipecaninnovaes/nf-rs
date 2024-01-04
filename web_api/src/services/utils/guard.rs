use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension, Json, extract::Path,
};

use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use jsonwebtoken::get_current_timestamp;

use crate::services::auth::{jwt::decode_jwt, struct_user::LoginTockenCheckModel};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub async fn guard(
    cookie_jar: CookieJar,
    Extension(_pool): Extension<Pool<Postgres>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    auth_value
                        .strip_prefix("Bearer ")
                        .map(|value| value.to_owned())
                })
        });

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let claims = decode_jwt(token)
        .map_err(|_err| {
            let json_error = ErrorResponse {
                status: "fail",
                message: "Unauthorized".to_owned(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        })?
        .claims;

    let email = claims.email;

    let q = "SELECT firstname, email FROM users WHERE email = $1";
    let user = sqlx::query_as::<_, LoginTockenCheckModel>(q)
        .bind(email)
        .fetch_optional(&_pool)
        .await
        .map_err(|e| {
            let json_error = ErrorResponse {
                status: "fail",
                message: format!("Error fetching user from database: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_error))
        })?;

    let user = user.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "The user belonging to this token no longer exists".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

// logout
#[allow(dead_code)]
pub async fn logout(cookie_jar: CookieJar) -> impl IntoResponse {
    let _ = cookie_jar.remove("token");
    (StatusCode::OK, "Logout success")
}

// extern token validation
pub async fn token_validation(path: Path<String>, Extension(_pool): Extension<Pool<Postgres>>) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {

    let claims = decode_jwt(path.0)
    .map_err(|_err| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Unauthorized".to_owned(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?.claims;
    let token_time = claims.exp as usize;
    let current_time = get_current_timestamp() as usize;
    if token_time < current_time {
        let json_error = ErrorResponse {
            status: "fail",
            message: "Token expired".to_owned(),
        };
        return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
    }
    Ok((StatusCode::OK, "Token valid"))
    
}