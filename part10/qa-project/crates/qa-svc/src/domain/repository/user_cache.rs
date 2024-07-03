use crate::domain::entity::UsersEntity;

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait UserCacheRepo: Send + Sync + 'static {
    async fn get(&self, username: &str) -> anyhow::Result<UsersEntity>;
    async fn set(&self, user: &UsersEntity) -> anyhow::Result<()>;
}
