// members table.
use std::time::Duration;
// MEMBERS_TABLE for members table
const MEMBERS_TABLE: &str = "members";

// MembersEntity for members table
#[derive(Debug, Default)]
pub struct MembersEntity {
    pub id: i64,
    pub openid: String,
    pub phone: String,
    pub nick: String,
    pub level: i64,
    pub points: i64,
    pub used_points: i64,
    pub expired_at: Duration,
    pub created_at: Duration,
    pub updated_at: Option<Duration>,
}

// impl table_name method for MembersEntity
impl MembersEntity {
    pub fn table_name(&self) -> String {
        MEMBERS_TABLE.to_string()
    }
}
