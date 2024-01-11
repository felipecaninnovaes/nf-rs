use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::services::empresas::{create::create_empresas, get::get_all_empresas};

pub fn empresas_routes() -> Router {
    let cors: CorsLayer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .route("/api/empresas/:userid", get(get_all_empresas))
        .route("/api/empresas/cadastrar", post(create_empresas))
        .layer(cors)
}
