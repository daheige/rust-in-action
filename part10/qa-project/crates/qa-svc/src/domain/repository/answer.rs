use crate::domain::entity::{AnswersEntity, QuestionsEntity};

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait AnswerRepo: Send + Sync + 'static {
    // 添加回答
    async fn add(&self, answer: AnswersEntity) -> anyhow::Result<i64>;

    // 删除回答
    async fn delete(&self, id: i64, username: &str) -> anyhow::Result<()>;

    // 更新回答内容
    async fn update(&self, id: i64, answer: AnswersEntity) -> anyhow::Result<()>;

    // 查看问题
    async fn detail(&self, id: i64) -> anyhow::Result<AnswersEntity>;

    // 点赞回答
    // 点赞后的数字是通过redis计数器增量数字+当前回答点赞数作为结果，立即返回，
    // 点赞数通过异步job方式落地到db，因此这里只需要返回成功与否。
    // 也就是说点赞数保持最终一致性就可以，不用做到百分百实时显示出来。
    // 点赞的同时，发送一个消息（点赞流水记录）到pulsar消息队列中，后续通过异步job方式写入db中。
    async fn agree(&self, id: i64, username: &str) -> anyhow::Result<bool>;

    // 回答列表
    async fn lists(
        &self,
        question_id: i64,
        page: i64,
        limit: i64,
        order_by: String,
    ) -> anyhow::Result<Vec<QuestionsEntity>>;
}
