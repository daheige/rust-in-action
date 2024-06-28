// 引入sqlx对应的时间格式数据类型
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

// QUESTIONS_TABLE for questions table
const QUESTIONS_TABLE: &str = "questions";

// QuestionsEntity for questions table
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuestionsEntity {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub read_count: u64,
    pub reply_count: u64,
    pub is_deleted: u8,
}

// 最新问题列表返回结果
#[derive(Debug, Default)]
pub struct LatestQuestions {
    pub questions: Vec<QuestionsEntity>,
    pub is_end: bool,
    pub last_id: Option<u64>,
}

// impl table_name method for QuestionsEntity
impl QuestionsEntity {
    pub fn table_name() -> String {
        QUESTIONS_TABLE.to_string()
    }
}
