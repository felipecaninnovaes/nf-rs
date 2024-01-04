use axum::{
    http::Method,
    routing::{get, post, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::services::nfe::{
    get::{get_all_nfe, get_nfe_by_dest, get_nfe_by_emit},
    upload::upload, delete::delete_nfe_by_id,
};

pub fn nfe_routes() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .route("/api/nfe", get(get_all_nfe))
        .route("/api/nfe/emit/:id", get(get_nfe_by_emit))
        .route("/api/nfe/dest/:id", get(get_nfe_by_dest))
        .route("/api/nfe/upload", post(upload))
        .route("/api/nfe/delete/:id", delete(delete_nfe_by_id))
        .layer(cors)
}
