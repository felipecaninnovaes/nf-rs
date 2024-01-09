use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension, Json, extract::Path,
};

use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use jsonwebtoken::get_current_timestamp;

use crate::services::auth::{jwt::decode_jwt, struct_user::LoginTockenCheckModel};

use super::gets::{get_cookie, Cookie, get_from_header, HeaderGet};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}


#[derive(Debug, Serialize, Clone)]
struct Guard {
    token: Option<Cookie>,
    user_id: Option<HeaderGet>
}

pub async fn guard(
    cookie_jar: CookieJar,
    Extension(_pool): Extension<Pool<Postgres>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {

    let guard = Guard {
        token: get_cookie(cookie_jar,&req, "Bearer"),
        user_id: get_from_header(&req, "IdUser")
    };

    let token = guard.token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail",
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let claims = decode_jwt(token.value.expect("Token not found"))
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