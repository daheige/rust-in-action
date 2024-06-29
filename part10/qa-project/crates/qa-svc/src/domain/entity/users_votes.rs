// 引入sqlx对应的时间格式数据类型
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// USERS_VOTES_TABLE for users_votes table
const USERS_VOTES_TABLE: &str = "users_votes";

// UsersFeedbackEntity for users_feedback table
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct UsersVotesEntity {
    pub id: u64,
    pub target_id: u64,
    pub target_type: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
}

// impl table_name method for UsersVotesEntity
impl UsersVotesEntity {
    pub fn table_name() -> String {
        USERS_VOTES_TABLE.to_string()
    }
}
