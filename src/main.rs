use axum::{
    // routing::{get, post},
    // http::StatusCode,
    // Json, 
    Router,
};
// use serde::{Serialize, Deserialize};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))   // for logging all
        .init();

    // build our application with a route
    // nest_service: used for static files
    let app = Router::new()
        // .route("/", get(root))
        .nest_service(
            "/", 
            ServeDir::new("static/main")
        )
        .nest_service(
            "/hwcj",
            ServeDir::new("static/hwcj")
        )
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    info!("Start serving...");
    axum::serve(listener, app).await.unwrap();
}

// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     "안경 사세요"
// }