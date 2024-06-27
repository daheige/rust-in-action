use crate::domain::entity::QuestionsEntity;

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

    // 问题列表
    async fn lists(
        &self,
        last_id: i64,
        limit: i64,
        order_by: String,
    ) -> anyhow::Result<Vec<QuestionsEntity>>;
}
