use axum::{
    http::Method,
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::services::users::get::get_all_users;



pub fn users_routers() -> Router {
    let cors: CorsLayer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .route("/api/user", get(get_all_users))
        .layer(cors)
}
