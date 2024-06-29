use super::sql_builder::gen_in_placeholder;
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
    async fn check_user_exist(&self, username: &str) -> anyhow::Result<bool> {
        let sql = format!(
            "select id from {} where username = ? limit 1",
            UsersEntity::table_name(),
        );

        // 将结果放入一个Result对应的元组中
        let res: (u64,) = sqlx::query_as(&sql)
            .bind(username.to_string())
            .fetch_one(&self.mysql_pool)
            .await?;
        Ok(res.0 > 0)
    }

    // 插入用户
    async fn add(&self, username: &str, password: &str) -> anyhow::Result<()> {
        let sql = format!(
            "insert into {} (username,password,openid,created_at) value(?,?,?,?)",
            UsersEntity::table_name()
        );
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let pwd = format!("{:x}", md5::compute(password.as_bytes()));
        let openid = Uuid::new_v4().to_string().replace("-", "");
        let affect_rows = sqlx::query(&sql)
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
    async fn fetch_one(&self, username: &str) -> anyhow::Result<UsersEntity> {
        let sql = format!(
            "select * from {} where username = ? limit 1",
            UsersEntity::table_name(),
        );

        // query_as将查询结果映射到结构体UserEntity中
        let user: UsersEntity = sqlx::query_as(&sql)
            .bind(username.to_string())
            .fetch_one(&self.mysql_pool)
            .await?;
        Ok(user)
    }

    // 批量查询用户信息
    async fn batch_users(&self, usernames: Vec<&str>) -> anyhow::Result<Vec<UsersEntity>> {
        // 将参数转换为(?,?)格式
        let parameters = gen_in_placeholder(usernames.len());
        let sql = format!(
            "select * from {} where username in ({})",
            UsersEntity::table_name(),
            parameters
        );

        println!("exec batch users sql:{}", sql);
        let mut query = sqlx::query_as(&sql);
        // 绑定参数
        for username in usernames {
            query = query.bind(username.to_string());
        }

        // 将查询结果集映射到Vec中
        let users: Vec<UsersEntity> = query.fetch_all(&self.mysql_pool).await?;
        Ok(users)
    }
}
