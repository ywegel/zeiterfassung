#![forbid(unsafe_code)]

pub mod configuration;
mod db;
mod error;
mod models;
mod repositories;
mod routes;

use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use axum::routing::post;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

pub use crate::db::connect_to_database;
pub use crate::repositories::region_repositories::RegionRepository;
pub use crate::repositories::region_repositories::SqliteRegionRepository;
use crate::routes::currently_active;
use crate::routes::history;
use crate::routes::start_timer;
use crate::routes::stop_timer;

#[derive(Clone)]
pub struct ApiContext {
    pub region_repository: Arc<dyn RegionRepository>,
}

pub fn app(api_context: ApiContext) -> Router {
    let static_frontend_files =
        ServeDir::new("./static").fallback(ServeFile::new("./static/index.html"));

    Router::new()
        .route("/hello_world", axum::routing::get(routes::hello_world))
        .route("/api/{region}/start", post(start_timer))
        .route("/api/{region}/stop", post(stop_timer))
        .route("/api/{region}/history", get(history))
        .route("/api/currently_active", get(currently_active))
        .with_state(api_context)
        .fallback_service(static_frontend_files)
}
