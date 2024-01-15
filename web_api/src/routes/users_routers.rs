use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::services::users::{select::select_users, update::update_user};

pub fn users_routers() -> Router {
    let cors: CorsLayer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .route("/api/user", get(select_users))
        .route("/api/user/update", post(update_user))
        .layer(cors)
}
