use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json, body::Body, Extension,
};

use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;
use sqlx::{Pool, Postgres};

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
                    auth_value.strip_prefix("nfemanager.token ").map(|value| value.to_owned())
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
    let user = sqlx::query_as::<_, LoginTockenCheckModel>(q).bind(email).fetch_optional(&_pool).await.map_err(|e| {
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



// use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};

// use crate::services::auth::{jwt::decode_jwt, struct_user::LoginUserModel};

// use super::api_error::APIError;

// pub async fn guard<T>(mut req: Request<T>, next: Next<T>) -> Result<Response,APIError> {

//     let token = req.headers().typed_get::<Authorization<Bearer>>()
//     .ok_or(APIError { message: "No Auth token found".to_owned(), status_code: StatusCode::BAD_REQUEST, error_code: Some(40)  })?.token().to_owned();

//     let claim = decode_jwt(token)
//     .map_err(|err| APIError { message: "Unauthorized".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41)  })?.claims;

//     let db = req.extensions().get::<DatabaseConnection>()
//     .ok_or(APIError { message: "Could not connect to database".to_owned(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)  })?;

//     let q = "SELECT email, password FROM users WHERE email = $1 AND password = $2";
//     let identity = sqlx::query_as::<_, LoginUserModel>(q).bind(user_data.email).bind(user_data.password).fetch_one(&_pool).await.map_err(|_| APIError { message: "Failed to login".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41) })?;


//     req.extensions_mut().insert(identity);

//     Ok(next.run(req).await)
// } 