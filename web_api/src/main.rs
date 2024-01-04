use axum::{
    extract::DefaultBodyLimit, // Add missing import statements
    http::Method,
    middleware,
    routing::{get, post},
    Extension,
    Router,
};
use dotenv::dotenv;
use nfe::modules::sql::connection_postgres::start_connection;
use tower_http::cors::{Any, CorsLayer};

mod services;
use services::{
    nfe::{
        get::{get_all_nfe, get_nfe_by_dest, get_nfe_by_emit},
        upload::upload,
    },
    utils::guard::guard,
};

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _pool = start_connection().await;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/api/nfe", get(get_all_nfe))
        .route("/api/nfe/emit/:id", get(get_nfe_by_emit))
        .route("/api/nfe/dest/:id", get(get_nfe_by_dest))
        .route("/api/nfe/upload", post(upload))
        .route_layer(middleware::from_fn(guard))
        .merge(routes::auth_routes::auth_routes())
        .layer(cors)
        .layer(Extension(_pool))
        .layer(DefaultBodyLimit::disable());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
