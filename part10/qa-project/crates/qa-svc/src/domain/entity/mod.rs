// 定义实体模块
mod answers;
mod questions;
mod users;

// 重新导入相关模块的数据类型
pub use answers::AnswersEntity;
pub use questions::QuestionsEntity;
pub use users::UsersEntity;
