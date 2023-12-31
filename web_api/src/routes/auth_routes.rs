use axum::{http::Method, routing::{post, get}, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::services::{auth::auth_handlers::{create_user, login_user_post}, utils::guard::token_validation};

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
