use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use nfe::modules::sql::{connection_postgres::start_connection, select::get_all_nfe};
use tower_http::cors::{Any, CorsLayer};

mod services;
use services::upload::upload;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = start_connection().await;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let json = get_all_nfe(&pool).await.unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/api/nfe", get(json))
        .route("/api/upload/nfe", post(upload))
        .layer(cors)
        .layer(DefaultBodyLimit::disable());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
