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

    // 点赞回答和取消点赞回答
    // 点赞后的数字是通过redis计数器增量数字+当前回答点赞数作为结果，立即返回，
    // 点赞数通过异步job方式落地到db，因此这里只需要返回成功与否。
    // 也就是说点赞数保持最终一致性就可以，不用做到百分百实时显示出来。
    // 点赞的同时，发送一个消息（点赞流水记录）到pulsar消息队列中，后续通过异步job方式写入db中。
    async fn handler_agree(&self, id: u64, username: &str, action: &str) -> anyhow::Result<bool>;

    // 回答列表
    async fn lists(
        &self,
        question_id: u64,
        page: u64,
        limit: u64,
        order_by: &str,
    ) -> anyhow::Result<AnswerListReply>;
}
