use crate::domain::entity::EntityReadCountData;

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait ReadCountRepo: Send + Sync + 'static {
    // 增加实体阅读数
    // 每个实体的阅读数通过redis hash incr计数器方式实现
    async fn incr(&self, data: &EntityReadCountData) -> anyhow::Result<u64>;

    // 处理每个实体的阅读数
    // 可以根据实际情况写入db表中或别的地方持久化存储
    async fn handler(&self, target_type: &str) -> anyhow::Result<()>;
}
