use serde::Deserialize;
use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Region {
    Aa1,
    Aa2,
    Aa3,
    Ac1,
    Ac2,
    Ac3,
}

#[derive(Debug, Serialize)]
pub struct CurrentlyActiveRegion {
    pub region: Option<Region>,
    pub duration: Option<i64>,
}

impl CurrentlyActiveRegion {
    pub fn nothing_active() -> CurrentlyActiveRegion {
        CurrentlyActiveRegion {
            region: None,
            duration: None,
        }
    }
}
