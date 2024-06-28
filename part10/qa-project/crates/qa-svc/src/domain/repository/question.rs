use crate::domain::entity::{LatestQuestions, QuestionsEntity};

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait QuestionRepo: Send + Sync + 'static {
    // 发表问题
    async fn add(&self, question: &QuestionsEntity) -> anyhow::Result<u64>;

    // 修改问题
    async fn update(&self, id: u64, question: &QuestionsEntity) -> anyhow::Result<()>;

    // 删除问题
    async fn delete(&self, id: u64, username: &str) -> anyhow::Result<()>;

    // 根据id查询问题实体
    async fn fetch_one(&self, id: u64) -> anyhow::Result<QuestionsEntity>;

    // 最新问题列表
    async fn latest_lists(&self, last_id: u64, limit: u64) -> anyhow::Result<LatestQuestions>;
}
