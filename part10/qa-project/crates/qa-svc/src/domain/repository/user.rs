use crate::domain::entity::UsersEntity;

// 通过async_trait宏标记定义异步方法
#[async_trait::async_trait]
pub trait UserRepo: Send + Sync + 'static {
    // 检查用户是否存在
    async fn check_user_exist(&self, username: &str) -> anyhow::Result<bool>;

    // 插入用户
    async fn add(&self, username: &str, password: &str) -> anyhow::Result<()>;

    // 查询单个用户信息
    async fn fetch_one(&self, username: &str) -> anyhow::Result<UsersEntity>;

    // 根据用户username批量获取用户信息
    async fn batch_users(&self, usernames: Vec<&str>) -> anyhow::Result<Vec<UsersEntity>>;
}
