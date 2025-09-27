use std::sync::Arc;

use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use backend::ApiContext;
use backend::SqliteRegionRepository;
use backend::app;
use http_body_util::BodyExt;
use sqlx::SqlitePool;
use tower::ServiceExt;

fn setup_api_context(pool: SqlitePool) -> ApiContext {
    let region_repository = Arc::new(SqliteRegionRepository::new(pool));
    ApiContext { region_repository }
}

#[sqlx::test]
async fn test_hello_world(pool: SqlitePool) {
    let app = app(setup_api_context(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/hello_world")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World!");
}
