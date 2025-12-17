use std::fmt::Display;
use std::fmt::Formatter;

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

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
