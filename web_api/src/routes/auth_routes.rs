use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::services::{
    auth::auth_handlers::login_user_post, users::create::create_user,
    utils::guard::token_validation,
};

pub fn auth_routes() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .route("/api/user/register", post(create_user))
        .route("/api/user/login", post(login_user_post))
        .route("/api/user/token/:token", get(token_validation))
        .layer(cors)
}
