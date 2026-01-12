use axum::{
    routing::{get},
    // http::StatusCode,
    // Json, 
    Router,
};
// use serde::{Serialize, Deserialize};
use tower_http::services::ServeDir;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    // nest_service: used for static files
    let app = Router::new()
        .route("/", get(root))
        .nest_service(
            "/hwcj",
            ServeDir::new("hwcj_static")
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    info!("Start serving...");
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "안경 사세요"
}