use axum::{
    body::Body,
    http::{header, Request}
};

use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Cookie {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct HeaderGet {
    pub name: Option<String>,
    pub value: Option<String>,
}

pub fn get_cookie(cookie_jar: CookieJar,req: &Request<Body>, name: &str) -> Option<Cookie> {
    let result = Cookie {
        name: Some(name.to_string()),
        value: {
            cookie_jar
    .get("token")
    .map(|cookie| cookie.value().to_string())
    .or_else(|| {
        req.headers()
            .get(header::AUTHORIZATION)
            .and_then(|auth_header| auth_header.to_str().ok())
            .and_then(|auth_value| {
                auth_value
                    .strip_prefix(format!("{} ", name).as_str())
                    .map(|value| value.to_owned())
            })
    })
        }
    };
    Some(result)
}

pub fn get_from_header(req: &Request<Body>, name: &str) -> Option<HeaderGet> {
    let result = HeaderGet {
        name: Some(name.to_string()),
        value: req.headers()
            .get(name).unwrap().to_str().ok().map(|value| value.to_owned())
    };
    Some(result)
}
