// 引入sqlx对应的时间格式数据类型
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// ANSWERS_TABLE for answers table
const ANSWERS_TABLE: &str = "answers";

// AnswersEntity for answers table
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct AnswersEntity {
    pub id: u64,
    pub question_id: u64,
    pub content: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub agree_count: u64,
    pub is_deleted: u8,
}

// impl table_name method for AnswersEntity
impl AnswersEntity {
    pub fn table_name() -> String {
        ANSWERS_TABLE.to_string()
    }
}
