use axum::Json;
use axum::extract::State;

use crate::ApiContext;
use crate::error::AppError;
use crate::models::region_history::DailyRegionSummary;

pub async fn daily_history(
    State(context): State<ApiContext>,
) -> Result<Json<Vec<DailyRegionSummary>>, AppError> {
    let all_daily_region_history = context.region_repository.get_daily_summary().await?;
    Ok(Json(all_daily_region_history))
}
