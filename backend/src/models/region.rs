use serde::Deserialize;
use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Region {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Action {
    Start,
    Stop,
}

#[derive(Debug, Deserialize)]
pub struct RegionAction {
    pub region: Region,
    pub action: Action,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub timestamp: String,
}
