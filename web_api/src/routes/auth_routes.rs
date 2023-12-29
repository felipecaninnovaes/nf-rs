use axum::{Router, http::Method, routing::post};
use tower_http::cors::{CorsLayer, Any};

use crate::services::auth::auth_handlers::{create_user, login_user_post};



pub fn auth_routes() -> Router {
    let cors = CorsLayer::new()
    .allow_methods([Method::POST])
    .allow_origin(Any);

    
    Router::new()
    .route("/api/user/register",post(create_user))
    .route("/api/user/login",post(login_user_post))
    .layer(cors)
}