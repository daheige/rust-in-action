use crate::domain::entity::UsersEntity;
use crate::domain::repository::UserCacheRepo;
use r2d2::Pool;
use redis::{Commands, ErrorKind,RedisError};
use redis::RedisResult;

struct UserCacheRepoImpl {
    redis_pool: Pool<redis::Client>,
}

pub fn new_user_cache(redis_pool: Pool<redis::Client>) -> impl UserCacheRepo{
    UserCacheRepoImpl{ redis_pool}
}

#[async_trait::async_trait]
impl UserCacheRepo for UserCacheRepoImpl {
    async fn get(&self, username: &str) -> anyhow::Result<UsersEntity> {
        let key = format!("user_info:{}", username);
        let mut conn = self.redis_pool.get().expect("get redis connection failed");
        let res :String = conn.get(&key)?;
        if res.is_empty(){
            let err = RedisError::from((
                ErrorKind::ResponseError,
                "user cache not found",
            ));
            return Err(anyhow::Error::from(err));
        }

        let user : UsersEntity = serde_json::from_str(&res)?;
        Ok(user)
    }

    async fn set(&self, user: &UsersEntity) -> anyhow::Result<()> {
        let key = format!("user_info:{}", user.username);
        let mut conn = self.redis_pool.get().expect("get redis connection failed");
        let value = serde_json::to_string(user)?;
        // println!("value:{:?}",value);
        let res: RedisResult<()> = conn.set_ex(&key, value, 24 * 3600);
        if res.is_err() {
            return Err(anyhow::Error::from(res.err().unwrap()));
        }

        Ok(())
    }
}
