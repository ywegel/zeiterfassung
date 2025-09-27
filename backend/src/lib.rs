#![forbid(unsafe_code)]

pub mod configuration;
mod db;
mod models;
mod repositories;
mod routes;

use std::sync::Arc;

use axum::Extension;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

pub use crate::db::connect_to_database;
pub use crate::repositories::region_repositories::RegionRepository;
pub use crate::repositories::region_repositories::SqliteRegionRepository;

#[derive(Clone)]
pub struct ApiContext {
    pub region_repository: Arc<dyn RegionRepository>,
}

pub fn app(api_context: ApiContext) -> Router {
    let static_frontend_files =
        ServeDir::new("./static").fallback(ServeFile::new("./static/index.html"));

    Router::new()
        .route("/hello_world", axum::routing::get(routes::hello_world))
        .layer(Extension(api_context))
        .fallback_service(static_frontend_files)
}
