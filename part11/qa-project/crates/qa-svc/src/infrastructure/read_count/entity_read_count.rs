use crate::domain::entity::{EntityReadCountData, QuestionsEntity};
use crate::domain::repository::ReadCountRepo;
use log::{error, info};
use r2d2::Pool;
use redis::Commands;
use std::ops::DerefMut;

// ReadCountRepoImpl实现ReadCountRepo trait的具体数据类型
struct ReadCountRepoImpl {
    redis_pool: Pool<redis::Client>,
    mysql_pool: sqlx::MySqlPool,
}

// 创建一个ReadCountRepo实例对象
pub fn new_read_count_repo(
    redis_pool: Pool<redis::Client>,
    mysql_pool: sqlx::MySqlPool,
) -> impl ReadCountRepo {
    ReadCountRepoImpl {
        redis_pool,
        mysql_pool,
    }
}

impl ReadCountRepoImpl {
    fn get_entity_table(&self, target_type: &str) -> String {
        if target_type == "question" {
            return QuestionsEntity::table_name();
        }

        return "unknown".to_string();
    }

    fn get_hash_key(&self, target_type: &str) -> String {
        format!("qa_sys:{}:read_count:hash", target_type)
    }

    // 将redis hash中实体增量计数器对应的数量，更新到对应的实体数据表中，并对应减少对应的数量
    async fn update_read_count(
        &self,
        target_id: i64,
        target_type: &str,
        increment: i64,
    ) -> anyhow::Result<()> {
        info!(
            "update target_id:{} target_type:{} read_count increment:{} begin",
            target_id, target_type, increment
        );
        let sql = format!(
            "update {} set read_count = read_count + ? where id = ?",
            self.get_entity_table(target_type),
        );

        let res = sqlx::query(&sql)
            .bind(increment)
            .bind(target_id)
            .execute(&self.mysql_pool)
            .await?;
        info!(
            "execute target_id:{} target_type:{} result:{:?}",
            target_id, target_type, res
        );
        if res.rows_affected() == 0 {
            return Err(anyhow::anyhow!(
                "failed to update target_id:{} target_type:{} read_count",
                target_id,
                target_type
            ));
        }

        // 更新redis hash 实体阅读数对应的计数器
        // redis hash 的field是实体id，value是阅读数
        let hash_key = self.get_hash_key(target_type);
        let mut conn = self.redis_pool.get().expect("get redis connection failed");
        let remain: i64 = conn.hincr(hash_key, target_id.to_string(), -increment)?;

        info!(
            "update current target_id:{} target_type:{} increment:{} hincry result:{} success",
            target_id, target_type, increment, remain
        );

        Ok(())
    }
}

#[async_trait::async_trait]
impl ReadCountRepo for ReadCountRepoImpl {
    async fn incr(&self, data: &EntityReadCountData) -> anyhow::Result<u64> {
        // redis hash 的field是target_id对应的字符串格式，value是阅读数增量计数器
        // 对问题阅读数增量计数器加1操作，后续可以通过job定期处理，将阅读数同步到db即可
        let hash_key = self.get_hash_key(&data.target_type);
        let mut conn = self.redis_pool.get().expect("get redis connection failed");
        let field = data.target_id.to_string();
        let increment: i64 = conn.hincr(&hash_key, field, data.count as i64)?;

        info!(
            "current target_id:{} target_type:{} hincr result:{}",
            data.target_id, data.target_type, increment
        );

        Ok(increment as u64)
    }

    async fn handler(&self, target_type: &str) -> anyhow::Result<()> {
        // 读取redis hash记录
        let hash_key = self.get_hash_key(target_type);
        let mut conn = self
            .redis_pool
            .get()
            .map_err(|err| anyhow::anyhow!("failed to get redis connection err:{}", err))?;

        // 通过hscan游标方式遍历数据
        let mut cursor: u64 = 0;
        let pattern = "*";
        let count = 500; // 每次扫描返回的数量

        loop {
            // 执行HSCAN命令
            // 当我们使用Redis hscan游标匹配数据时，
            // 第一个元素是field，第二个元素是field对应的value值
            let result: (u64, Vec<(String, String)>) = redis::cmd("HSCAN")
                .arg(&hash_key)
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(count)
                .query(conn.deref_mut())?;

            let (new_cursor, records) = result;
            for (field, value) in records.iter() {
                let target_id: i64 = field.parse().unwrap_or(0); // 当前文章id
                let increment: i64 = value.parse().unwrap_or(0); // 当前文章增量计数器
                if increment == 0 || target_id <= 0 {
                    continue;
                }

                println!(
                    "exec target_id:{} target_type:{} read_count increment:{} begin",
                    target_id, target_type, increment
                );
                let res = self
                    .update_read_count(target_id, target_type, increment)
                    .await;
                if res.is_err() {
                    error!("failed to update error:{}", res.err().unwrap());
                } else {
                    println!(
                        "handler target_id:{} target_type:{} read_count increment:{} success",
                        target_id, target_type, increment
                    );
                }
            }

            // 当游标回到0时表示遍历完成
            if cursor == 0 {
                break;
            }

            cursor = new_cursor;
        }

        Ok(())
    }
}
