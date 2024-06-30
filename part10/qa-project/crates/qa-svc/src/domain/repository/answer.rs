use crate::domain::entity::{AnswerListReply, AnswersEntity};

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait AnswerRepo: Send + Sync + 'static {
    // 添加回答
    async fn add(&self, answer: &AnswersEntity) -> anyhow::Result<u64>;

    // 删除回答
    async fn delete(&self, id: u64, username: &str) -> anyhow::Result<()>;

    // 更新回答内容
    async fn update(&self, id: u64, content: &str, updated_by: &str) -> anyhow::Result<()>;

    // 查询回答信息
    async fn fetch_one(&self, id: u64) -> anyhow::Result<AnswersEntity>;

    // 回答列表
    async fn lists(
        &self,
        question_id: u64,
        page: u64,
        limit: u64,
        order_by: &str,
    ) -> anyhow::Result<AnswerListReply>;
}
