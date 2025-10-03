use std::net::SocketAddr;
use std::sync::Arc;

use axum::serve;
use backend::ApiContext;
use backend::SqliteRegionRepository;
use backend::app;
use backend::configuration::Configuration;
use backend::configuration::ConfigurationError;
use backend::configuration::load_configuration;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigurationError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = load_configuration(None).map_err(|e: ConfigurationError| {
        eprintln!("Failed to load configuration: {}", e);
        e
    })?;

    let api_context = api_context(&config).await.map_err(|e| {
        eprintln!("Failed to create API context: {}", e);
        e
    })?;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.application_port));

    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        eprintln!("Failed to bind to address {}: {}", addr, e);
        AppError::Io(e)
    })?;

    println!("Server running on http://{}", addr);
    serve(listener, app(api_context)).await.map_err(|e| {
        eprintln!("Server error: {}", e);
        AppError::Io(e)
    })?;

    Ok(())
}

async fn api_context(config: &Configuration) -> Result<ApiContext, AppError> {
    let pool = backend::db::connect_to_database(&config.database_url).await?;
    let region_repository = Arc::new(SqliteRegionRepository::new(pool));
    Ok(ApiContext { region_repository })
}
