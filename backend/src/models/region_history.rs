use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;

use crate::models::region::Region;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RegionHistory {
    pub region: Region,
    pub start_time: DateTime<Utc>,
    pub stop_time: Option<DateTime<Utc>>,
    pub duration: Option<i64>,
}
