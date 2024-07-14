use crate::domain::entity::UserSessionEntity;

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait UserSessionRepo: Send + Sync + 'static {
    async fn get(&self, key: &str) -> anyhow::Result<UserSessionEntity>;
    async fn set(&self, key: &str, user: &UserSessionEntity, seconds: u64) -> anyhow::Result<()>;
    async fn del(&self, key: &str) -> anyhow::Result<()>;
}
