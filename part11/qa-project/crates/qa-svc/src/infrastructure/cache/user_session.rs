use crate::domain::entity::UserSessionEntity;
use crate::domain::repository::UserSessionRepo;
use r2d2::Pool;
use redis::RedisResult;
use redis::{Commands, ErrorKind, RedisError};

struct UserCacheRepoImpl {
    redis_pool: Pool<redis::Client>,
}

pub fn new_user_cache(redis_pool: Pool<redis::Client>) -> impl UserSessionRepo {
    UserCacheRepoImpl { redis_pool }
}

#[async_trait::async_trait]
impl UserSessionRepo for UserCacheRepoImpl {
    async fn get(&self, key: &str) -> anyhow::Result<UserSessionEntity> {
        let mut conn = self.redis_pool.get()?;
        let res: RedisResult<String> = conn.get(key);
        if let Err(err) = res {
            let kind = err.kind();
            return match kind {
                ErrorKind::TypeError => {
                    let err = RedisError::from((ErrorKind::TypeError, "user session not found"));
                    Err(anyhow::Error::from(err))
                }
                _ => {
                    let err = RedisError::from((
                        ErrorKind::ResponseError,
                        "unknown error",
                        format!("{}", err),
                    ));
                    Err(anyhow::Error::from(err))
                }
            };
        }

        let res = res.unwrap();
        if res.is_empty() {
            let err = RedisError::from((ErrorKind::ResponseError, "user session is empty"));
            return Err(anyhow::Error::from(err));
        }

        let user: UserSessionEntity = serde_json::from_str(&res)?;
        Ok(user)
    }

    async fn set(&self, key: &str, user: &UserSessionEntity, seconds: u64) -> anyhow::Result<()> {
        let mut conn = self.redis_pool.get()?;
        let value = serde_json::to_string(user)?;
        // println!("value:{:?}",value);
        let _: () = conn.set_ex(key, value, seconds)?;
        Ok(())
    }

    async fn del(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.redis_pool.get()?;

        let _: () = conn.del(key)?;

        Ok(())
    }
}
