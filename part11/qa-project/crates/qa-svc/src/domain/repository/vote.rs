// 引入点赞消息模块
use crate::domain::entity::VoteMessage;

// 引入标准库模块
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait UserVoteRepo: Send + Sync + 'static {
    // 判断用户是否对某个实体对象已点赞
    async fn is_voted(
        &self,
        target_id: u64,
        target_type: &str,
        username: &str,
    ) -> anyhow::Result<bool>;

    // 判断用户对一批实体对象是否已点赞
    async fn is_batch_voted(
        &self,
        target_id: &Vec<u64>,
        target_type: &str,
        username: &str,
    ) -> anyhow::Result<HashMap<u64, bool>>;

    // 发送用户点赞消息
    async fn publish(&self, msg: VoteMessage) -> anyhow::Result<bool>;

    // 根据实体类型异步消费用户点赞消息，实现点赞数增量更新和记录点赞明细
    async fn consumer(&self, target_type: &str, exit: Arc<RwLock<bool>>) -> anyhow::Result<()>;
}
