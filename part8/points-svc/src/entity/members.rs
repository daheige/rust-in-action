// 引入serde序列化和反序列化trait
use serde::{Deserialize, Serialize};
// 引入sqlx对应的时间格式数据类型
use sqlx::types::chrono::NaiveDateTime;

// MEMBERS_TABLE for members table
const MEMBERS_TABLE: &str = "members";

// MembersEntity for members table
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MembersEntity {
    pub id: u64,        // 自增id
    pub openid: String, // 用户唯一标识
    pub phone: String,
    pub nick: String,
    pub level: u8,
    pub points: u64,      // 用户当前总积分
    pub used_points: u64, // 用户已使用的积分
    pub expired_at: NaiveDateTime,
    pub created_at: NaiveDateTime,         // 积分创建时间
    pub updated_at: Option<NaiveDateTime>, // 更新时间
}

// impl table_name method for MembersEntity
impl MembersEntity {
    pub fn table_name() -> String {
        MEMBERS_TABLE.to_string()
    }
}
