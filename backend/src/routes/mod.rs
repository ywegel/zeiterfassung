use axum::Json;
use axum::extract::Path;
use axum::extract::State;

use crate::ApiContext;
use crate::error::AppError;
use crate::models::region::CurrentlyActiveRegion;
use crate::models::region::Region;
use crate::models::region_history::RegionHistory;

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[axum_macros::debug_handler]
pub async fn start_timer(
    Path(region): Path<Region>,
    State(context): State<ApiContext>,
) -> Result<(), AppError> {
    let result = context.region_repository.start_timer(region).await?;
    Ok(result)
}

#[derive(serde::Serialize)]
pub struct StopTimerResponse {
    duration: i64,
}

pub async fn stop_timer(
    Path(region): Path<Region>,
    State(context): State<ApiContext>,
) -> Result<Json<StopTimerResponse>, AppError> {
    let duration = context.region_repository.stop_timer(region).await?;
    Ok(Json(StopTimerResponse { duration }))
}

pub async fn history_by_region(
    Path(region): Path<Region>,
    State(context): State<ApiContext>,
) -> Result<Json<Vec<RegionHistory>>, AppError> {
    let region_history = context
        .region_repository
        .get_history_by_region(region)
        .await?;
    Ok(Json(region_history))
}

pub async fn currently_active(
    State(context): State<ApiContext>,
) -> Result<Json<CurrentlyActiveRegion>, AppError> {
    let result = context.region_repository.currently_active_timer().await?;
    Ok(Json(result))
}

#[cfg(test)]
mod tests {
    use crate::routes::hello_world;

    #[tokio::test]
    async fn test_hello_world_handler() {
        let result = hello_world().await;
        assert_eq!(result, "Hello, World!");
    }
}
