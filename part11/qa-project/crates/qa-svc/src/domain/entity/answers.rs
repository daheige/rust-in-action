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
}

#[derive(Debug, Default)]
pub struct AnswerListReply {
    pub answers: Vec<AnswersEntity>,
    pub total: u64,
    pub total_page: u64,
    pub page_size: u64,
    pub current_page: u64,
    pub is_end: bool,
}

impl AnswerListReply {
    pub fn new(answers: Vec<AnswersEntity>, total: u64, page: u64, limit: u64) -> Self {
        let total_page = (total as f64 / limit as f64).ceil() as u64;
        Self {
            answers,
            total: total,
            total_page,
            page_size: limit,
            current_page: page,
            is_end: page * limit >= total,
        }
    }
}

// impl table_name method for AnswersEntity
impl AnswersEntity {
    pub fn table_name() -> String {
        ANSWERS_TABLE.to_string()
    }
}
