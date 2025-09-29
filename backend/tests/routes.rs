use std::sync::Arc;

use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use backend::ApiContext;
use backend::SqliteRegionRepository;
use backend::app;
use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::SqlitePool;
use sqlx::types::JsonValue;
use tower::ServiceExt;

use crate::utils::RouterExt;

mod utils;

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

#[sqlx::test]
async fn test_start_timer(pool: SqlitePool) {
    // Cloning the pool results in a new pool that is tied to the same shared
    // connection pool. We need to do this, because our application requires an
    // owned pool, but we also need a pool for testing.
    let application_pool = pool.clone();

    // Given
    let app = app(setup_api_context(application_pool));

    // When
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ac1/start")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: response
    assert_eq!(response.status(), StatusCode::OK);

    // Then: Internal state
    let history = sqlx::query!(
        "SELECT region, start_time, stop_time FROM region_history WHERE region = 'ac1'"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(history.len(), 1);
    assert_eq!(history[0].region, "ac1");
    assert!(history[0].stop_time.is_none());
    assert!(history[0].start_time.len() > 0);
}

#[sqlx::test]
async fn test_stop_timer(pool: SqlitePool) {
    // Cloning the pool results in a new pool that is tied to the same shared
    // connection pool. We need to do this, because our application requires an
    // owned pool, but we also need a pool for testing.
    let application_pool = pool.clone();

    // Given
    let mut app = app(setup_api_context(application_pool));

    app.call_request(
        Request::builder()
            .uri("/api/ac1/start")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    // We need to wait for more than 1s, as the duration calculated by sqlite is in
    // seconds
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

    // When
    let response = app
        .call_request(
            Request::builder()
                .uri("/api/ac1/stop")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    // Then: response
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let parsed: Value = serde_json::from_slice(body.iter().as_slice()).unwrap();
    let duration = parsed["duration"].as_i64().unwrap();
    assert!(duration >= 1, "The duration should be around 1 second");

    // Then: Internal state
    let history = sqlx::query!(
        "SELECT region, start_time, stop_time, duration FROM region_history WHERE region = 'ac1'"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(history.len(), 1);
    assert_eq!(history[0].region, "ac1");
    assert!(history[0].stop_time.is_some());
    assert!(history[0].start_time.len() > 0);
    assert!(history[0].duration.is_some());
    assert!(history[0].duration.unwrap() >= 1);
}

#[sqlx::test]
async fn test_stop_timer_when_no_time_was_started(pool: SqlitePool) {
    // Cloning the pool results in a new pool that is tied to the same shared
    // connection pool. We need to do this, because our application requires an
    // owned pool, but we also need a pool for testing.
    let application_pool = pool.clone();

    // Given
    let mut app = app(setup_api_context(application_pool));

    // When
    let response = app
        .call_request(
            Request::builder()
                .uri("/api/ac1/stop")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    // Then: response
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"No timer is running for the region");

    // Then: Internal state
    let history = sqlx::query!(
        "SELECT region, start_time, stop_time, duration FROM region_history WHERE region = 'ac1'"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(history.len(), 0);
}

// TODO: test what happens, if we start a time for a region, that already has a
// running timer

// TODO: test what happens, if we start a timer while another timer for another
// region is running

// TODO: Test the general history route, instead of the region history route
#[sqlx::test]
async fn test_history_is_empty_if_no_timer_ever_existed(pool: SqlitePool) {
    // Cloning the pool results in a new pool that is tied to the same shared
    // connection pool. We need to do this, because our application requires an
    // owned pool, but we also need a pool for testing.
    let application_pool = pool.clone();

    // Given
    let app = app(setup_api_context(application_pool));

    // When
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ac1/history")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: response
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let history = serde_json::from_slice::<Vec<TestRegionHistory>>(body.iter().as_slice()).unwrap();
    assert!(history.is_empty());

    // Then: Internal state
    let history = sqlx::query!(
        "SELECT region, start_time, stop_time, duration FROM region_history WHERE region = 'ac1'"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(history.len(), 0);
}

#[derive(Debug, serde::Deserialize)]
pub struct TestRegionHistory {
    pub region: String,
    pub start_time: DateTime<Utc>,
    pub stop_time: Option<DateTime<Utc>>,
    pub duration: Option<i64>,
}

#[sqlx::test]
async fn test_history_by_region_returns_a_started_timer(pool: SqlitePool) {
    let mut app = app(setup_api_context(pool));

    app.call_request(
        Request::builder()
            .uri("/api/ac1/start")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    // When
    let response = app
        .call_request(
            Request::builder()
                .uri("/api/ac1/history")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    // Then: response
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let history = serde_json::from_slice::<Vec<TestRegionHistory>>(body.iter().as_slice()).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].region, "ac1");
    assert!(history[0].start_time < Utc::now());
    assert!(
        history[0].start_time
            > Utc::now()
                .checked_sub_signed(TimeDelta::seconds(5))
                .unwrap()
    );
    assert!(history[0].stop_time.is_none());
    assert!(history[0].duration.is_none());
}

#[sqlx::test]
async fn test_history_by_region_returns_a_stopped_timer(pool: SqlitePool) {
    let mut app = app(setup_api_context(pool));

    app.call_request(
        Request::builder()
            .uri("/api/ac1/start")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    // We need to wait for more than 1s, as the duration calculated by sqlite is in
    // seconds
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

    app.call_request(
        Request::builder()
            .uri("/api/ac1/stop")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    // When
    let response = app
        .call_request(
            Request::builder()
                .uri("/api/ac1/history")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    // Then: response
    let now = Utc::now();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let history = serde_json::from_slice::<Vec<TestRegionHistory>>(body.iter().as_slice()).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].region, "ac1");
    assert!(history[0].start_time < now);
    assert!(history[0].start_time > now.checked_sub_signed(TimeDelta::seconds(5)).unwrap());
    assert!(history[0].stop_time.is_some());
    assert!(history[0].stop_time.unwrap() < now);
    assert!(history[0].stop_time.unwrap() > now.checked_sub_signed(TimeDelta::seconds(5)).unwrap());
    assert!(history[0].stop_time.unwrap() > history[0].start_time);
    assert!(history[0].duration.is_some());
    assert!(history[0].duration.unwrap() >= 1);
}

#[sqlx::test]
async fn test_currently_active_timer_when_no_timer_exists(pool: SqlitePool) {
    let app = app(setup_api_context(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/currently_active")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Then: response
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let history = serde_json::from_slice::<Value>(body.iter().as_slice()).unwrap();
    assert_eq!(history["region"], JsonValue::Null);
    assert_eq!(history["duration"], JsonValue::Null);
}

#[sqlx::test]
async fn test_currently_active_timer_when_none_is_active(pool: SqlitePool) {
    // Given
    let mut app = app(setup_api_context(pool));

    app.call_request(
        Request::builder()
            .uri("/api/ac1/start")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    // We need to wait for more than 1s, as the duration calculated by sqlite is in
    // seconds
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

    app.call_request(
        Request::builder()
            .uri("/api/ac1/stop")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    // When
    let response = app
        .call_request(
            Request::builder()
                .uri("/api/currently_active")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    // Then: response
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let history = serde_json::from_slice::<Value>(body.iter().as_slice()).unwrap();
    assert_eq!(history["region"], JsonValue::Null);
    assert_eq!(history["duration"], JsonValue::Null);
}

#[sqlx::test]
async fn test_currently_active_timer_when_timer_is_active(pool: SqlitePool) {
    // Given
    let mut app = app(setup_api_context(pool));

    app.call_request(
        Request::builder()
            .uri("/api/ac1/start")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    // We need to wait for more than 1s, as the duration calculated by sqlite is in
    // seconds
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

    // When
    let response = app
        .call_request(
            Request::builder()
                .uri("/api/currently_active")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await;

    // Then: response
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let history = serde_json::from_slice::<Value>(body.iter().as_slice()).unwrap();
    assert_eq!(history["region"], "ac1");
    assert_eq!(history["duration"], 1)
}
