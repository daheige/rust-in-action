use crate::domain::entity::UsersEntity;

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait UserRepo: Send + Sync + 'static {
    // 检查用户是否存在
    async fn check_user(&self, username: &str) -> anyhow::Result<bool>;

    // 插入用户
    async fn insert_user(&self, username: &str, password: &str) -> anyhow::Result<()>;

    // 查询用户信息
    async fn query_user(&self, username: &str) -> anyhow::Result<UsersEntity>;
}
