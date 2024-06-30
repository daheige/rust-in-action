use crate::domain::entity::VoteMessage;

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait VoteMessageRepo: Send + Sync + 'static {
    // 发送用户点赞消息
    async fn publish(&self, msg: &VoteMessage) -> anyhow::Result<bool>;

    // 根据实体类型，消费用户点赞的消息
    async fn consumer(&self, target_type: &str);
}
