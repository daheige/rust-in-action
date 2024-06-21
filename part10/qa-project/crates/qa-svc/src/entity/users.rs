// 引入sqlx对应的时间格式数据类型
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// USERS_TABLE for users table
const USERS_TABLE: &str = "users";

// UsersEntity for users table
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct UsersEntity {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub nick: String,
    pub openid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

// impl table_name method for UsersEntity
impl UsersEntity {
    pub fn table_name() -> String {
        USERS_TABLE.to_string()
    }
}
