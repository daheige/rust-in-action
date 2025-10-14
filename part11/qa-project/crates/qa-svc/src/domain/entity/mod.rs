use serde::{Deserialize, Serialize};
mod answers;
mod entity_read_count;
mod questions;
mod users;
mod users_votes;
mod vote_message;

// 总数对象
#[derive(Debug, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct CountInfo {
    pub cnt: i64, // 数量总数在mysql底层是bigint类型
}

// 重新导出这些实体结构体
pub use answers::{AnswerListReply, AnswersEntity};
pub use entity_read_count::EntityReadCountData;
pub use questions::{LatestQuestionsReply, QuestionsEntity};
pub use users::{UserSessionEntity, UsersEntity};
pub use users_votes::UsersVotesEntity;
pub use vote_message::VoteMessage;
