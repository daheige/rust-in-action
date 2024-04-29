use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// POINTS_DETAILS_TABLE for points_details table
const POINTS_DETAILS_TABLE: &str = "points_details";

// PointsDetailsEntity for points_details table
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PointsDetailsEntity {
    pub id: u64,
    pub openid: String,
    pub points: u64,
    pub action: String,
    pub reason: String,
    pub created_at: NaiveDateTime,
}

// impl table_name method for PointsDetailsEntity
impl PointsDetailsEntity {
    pub fn table_name() -> String {
        POINTS_DETAILS_TABLE.to_string()
    }
}
