use axum::{
    extract::DefaultBodyLimit, // Add missing import statements
    http::Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

mod services;
use services::{upload::upload, get::{get_all_nfe, get_nfe_by_emit, get_nfe_by_dest}};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/api/nfe", get(get_all_nfe))
        .route("/api/nfe/emit/:id", get(get_nfe_by_emit))
        .route("/api/nfe/dest/:id", get(get_nfe_by_dest))
        .route("/api/upload/nfe", post(upload))
        .layer(cors)
        .layer(DefaultBodyLimit::disable());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
