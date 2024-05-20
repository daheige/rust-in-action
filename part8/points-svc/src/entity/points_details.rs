use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// POINTS_DETAILS_TABLE for points_details table
const POINTS_DETAILS_TABLE: &str = "points_details";

// PointsDetailsEntity for points_details table
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PointsDetailsEntity {
    pub id: u64,                   // 自增id
    pub openid: String,            // 用户唯一标识
    pub points: u64,               // 积分数
    pub action: String,            // 增加或扣减动作，add表示增加，sub表示扣除
    pub reason: String,            // 积分操作理由
    pub created_at: NaiveDateTime, // 积分创建时间
}

// impl table_name method for PointsDetailsEntity
impl PointsDetailsEntity {
    pub fn table_name() -> String {
        POINTS_DETAILS_TABLE.to_string()
    }
}
