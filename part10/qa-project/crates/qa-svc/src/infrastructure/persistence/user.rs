use crate::domain::entity::users::UsersEntity;
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

#[tonic::async_trait]
impl UserRepo for UserRepoImpl {
    // 检查用户是否存在
    async fn check_user(&self, username: &str) -> anyhow::Result<bool> {
        let sql = format!(
            "select * from {} where username = ?",
            UsersEntity::table_name(),
        );

        // query_as将其映射到结构体UserEntity中
        let user: UsersEntity = sqlx::query_as(&sql)
            .bind(username)
            .fetch_one(&self.mysql_pool)
            .await?;

        Ok(user.id > 0)
    }

    // 插入用户
    async fn insert_user(&self, username: &str, password: &str) -> anyhow::Result<()> {
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
}
