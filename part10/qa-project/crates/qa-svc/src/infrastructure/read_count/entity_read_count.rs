use crate::domain::entity::{EntityReadCountData, QuestionsEntity};
use crate::domain::repository::ReadCountRepo;
use log::info;
use r2d2::Pool;
use redis::Commands;

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
    async fn update_read_count(&self, target_id: i64, target_type: &str, increment: i64) {
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
            .await;
        info!(
            "execute target_id:{} target_type:{} result:{:?}",
            target_id, target_type, res
        );
        if res.is_ok() {
            // 更新redis hash 实体阅读数对应的计数器
            // redis hash 的field是实体id，value是阅读数
            let hash_key = self.get_hash_key(target_type);
            let mut conn = self.redis_pool.get().expect("get redis connection failed");
            let remain: i64 = conn
                .hincr(hash_key, target_id.to_string(), -increment)
                .expect("redis hincr failed");
            info!(
                "current target_id:{} target_type:{} hincry result:{}",
                target_id, target_type, remain
            );
        }

        info!(
            "update target_id:{} target_type:{} read_count increment:{} end",
            target_id, target_type, increment
        );
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
        let mut conn = self.redis_pool.get().expect("get redis connection failed");

        // 返回对应的key val key val...格式，对应的是id read_count增量计数器的字符串格式
        let res: redis::Iter<String> = conn.hscan_match(hash_key, "*").unwrap();
        let records: Vec<String> = res.collect();
        let len = records.len();
        if len == 0 {
            return Ok(());
        }

        // 执行实体阅读数增量更新操作
        let mut i: usize = 0;
        while i < len {
            // 当前实体id
            let target_id: i64 = records.get(i).unwrap().parse().unwrap();
            // 当前实体增量计数器
            let increment: i64 = records.get(i + 1).unwrap().parse().unwrap();
            i += 2; // 这里i的值第一次迭代时 i = 0，第二次迭代 i = 2,依次类推
            if increment == 0 || target_id <= 0 {
                continue;
            }

            info!(
                "target_id:{} target_type:{}read_count increment:{}",
                target_id, target_type, increment
            );
            self.update_read_count(target_id, target_type, increment)
                .await;
        }

        Ok(())
    }
}
