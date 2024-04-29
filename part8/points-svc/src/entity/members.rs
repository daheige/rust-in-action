use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// MEMBERS_TABLE for members table
const MEMBERS_TABLE: &str = "members";

// MembersEntity for members table
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MembersEntity {
    pub id: u64,
    pub openid: String,
    pub phone: String,
    pub nick: String,
    pub level: u8,
    pub points: u64,
    pub used_points: u64,
    pub expired_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

// impl table_name method for MembersEntity
impl MembersEntity {
    pub fn table_name() -> String {
        MEMBERS_TABLE.to_string()
    }
}
