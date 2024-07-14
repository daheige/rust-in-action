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

// user login session entity
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserSessionEntity {
    pub uid: u64,
    pub username: String,
    pub openid: String,
    pub login_time: String,
    pub expire_time: String,
}

// user session error
pub enum UserSessionError {
    NotFound,        // session not found
    Empty,           // session is empty
    Unknown(String), // unknown error
}

// user token error
pub enum UserTokenError {
    Empty,                     // token is empty
    Invalid,                   // token invalid
    DecryptFailed(String),     // token decrypt failed
    ExpireTimeInvalid(String), // token expire_time parse error
    Expired,                   // token has expired
}
