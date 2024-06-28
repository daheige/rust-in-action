use serde::{Deserialize, Serialize};
mod answers;
mod questions;
mod users;
mod users_feedback;

// 总数对象
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct CountInfo {
    pub cnt: u64,
}

// 重新导出这些实体结构体
pub use answers::AnswersEntity;
pub use questions::{LatestQuestions, QuestionsEntity};
pub use users::UsersEntity;
pub use users_feedback::UsersFeedbackEntity;
