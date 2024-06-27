// 引入sqlx对应的时间格式数据类型
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// USERS_FEEDBACK_TABLE for users_feedback table
const USERS_FEEDBACK_TABLE: &str = "users_feedback";

// UsersFeedbackEntity for users_feedback table
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct UsersFeedbackEntity {
	pub id: u64,
	pub target_id: u64,
	pub target_type: String,
	pub created_by: String,
	pub created_at: NaiveDateTime,
}

// impl table_name method for UsersFeedbackEntity
impl UsersFeedbackEntity {
	pub fn table_name() -> String {
		USERS_FEEDBACK_TABLE.to_string()
	}
}