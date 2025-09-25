mod routes;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn app() -> Router {
    let static_frontend_files = ServeDir::new("./static")
        .fallback(ServeFile::new("./static/index.html"));

    Router::new()
        .route("/hello_world", axum::routing::get(routes::hello_world))
        .fallback_service(static_frontend_files)
}
