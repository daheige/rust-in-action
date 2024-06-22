use crate::domain::entity::UsersEntity;
use crate::domain::repository::UserRepo;
use chrono::Local;
use uuid::Uuid;

// UserRepo 具体实现
struct UserRepoImpl {
    mysql_pool: sqlx::MySqlPool,
}

pub fn new_user_repo(mysql_pool: sqlx::MySqlPool) -> impl UserRepo {
    let user_repo = UserRepoImpl { mysql_pool };
    user_repo
}

#[async_trait::async_trait]
impl UserRepo for UserRepoImpl {
    // 检查用户是否存在
    async fn check_user(&self, username: &str) -> anyhow::Result<bool> {
        let user = self.query_user(username).await?;
        Ok(user.id > 0)
    }

    // 插入用户
    async fn add(&self, username: &str, password: &str) -> anyhow::Result<()> {
        let sql = r#"insert into users (username,password,openid,created_at) value(?,?,?,?)"#;
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let pwd = format!("{:x}", md5::compute(password.as_bytes()));
        let openid = Uuid::new_v4().to_string().replace("-", "");
        let affect_rows = sqlx::query(sql)
            .bind(username)
            .bind(pwd)
            .bind(openid)
            .bind(created_at)
            .execute(&self.mysql_pool)
            .await?;

        let id = affect_rows.last_insert_id();
        println!("current insert user id = {}", id);

        Ok(())
    }

    // 查询用户信息
    async fn query_user(&self, username: &str) -> anyhow::Result<UsersEntity> {
        let sql = format!(
            "select * from {} where username = ?",
            UsersEntity::table_name(),
        );

        // query_as将其映射到结构体UserEntity中
        let user: UsersEntity = sqlx::query_as(&sql)
            .bind(username)
            .fetch_one(&self.mysql_pool)
            .await?;
        Ok(user)
    }

    // 批量查询用户信息
    async fn batch_users(&self, usernames: Vec<&str>) -> anyhow::Result<Vec<UsersEntity>> {
        let parameters = usernames
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");
        let sql = format!(
            "select * from {} where username in ({})",
            UsersEntity::table_name(),
            parameters
        );

        let mut query = sqlx::query_as(&sql);
        // 绑定参数
        for username in &usernames {
            query = query.bind(username);
        }

        // 通过sqlx提供的query_as方法将查询结果集映射到Vec中
        let users: Vec<UsersEntity> = query.fetch_all(&self.mysql_pool).await?;
        Ok(users)
    }
}
