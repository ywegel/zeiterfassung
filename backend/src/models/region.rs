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
