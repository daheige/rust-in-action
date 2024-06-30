use crate::domain::entity::{AnswersEntity, UsersVotesEntity, VoteMessage};
use crate::domain::repository::VoteRepo;
use chrono::Local;
use log::info;
use pulsar::{Pulsar, TokioExecutor};
use std::ops::DerefMut;

struct VoteRepoImpl {
    mysql_pool: sqlx::MySqlPool,
    pulsar_client: Pulsar<TokioExecutor>,
}

pub fn new_vote_repo(
    mysql_pool: sqlx::MySqlPool,
    pulsar_client: Pulsar<TokioExecutor>,
) -> impl VoteRepo {
    VoteRepoImpl {
        mysql_pool,
        pulsar_client,
    }
}

impl VoteRepoImpl {
    // 回答点赞
    async fn vote(&self, id: u64, username: &str) -> anyhow::Result<bool> {
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let sql = format!(
            r#"insert into {} (target_id,target_type,created_by,created_at) value (?,?,?,?)"#,
            UsersVotesEntity::table_name(),
        );
        println!("insert vote sql:{}", sql);

        let mysql_pool = &self.mysql_pool;
        let mut tx = mysql_pool.begin().await?;
        // 插入积分明细记录
        let affect_res = sqlx::query(&sql)
            .bind(id)
            .bind("answer")
            .bind(username)
            .bind(created_at)
            // 在sqlx 0.7版本以上，execute这里需要对tx进行解引用并获取内部DB的可变引用connection
            .execute(tx.deref_mut())
            .await?;
        info!(
            "insert vote detail affect_rows:{}",
            affect_res.rows_affected()
        );

        // 更新当前回答点赞数
        let sql = format!(
            r#"update {} set agree_count = agree_count + ?,updated_at = ? where id = ?"#,
            AnswersEntity::table_name(),
        );
        let updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("vote update sql:{}", sql);
        let affect_res = sqlx::query(&sql)
            .bind(1)
            .bind(updated_at)
            .bind(id)
            .execute(tx.deref_mut())
            .await?;
        info!(
            "update answer vote affect_rows:{}",
            affect_res.rows_affected()
        );

        // 提交事务
        tx.commit().await?;
        Ok(true)
    }

    async fn has_voted(&self, id: u64, username: &str) -> anyhow::Result<bool> {
        let sql = format!(
            "select id from {} where target_id = ? and target_type = ? and created_by = ?",
            UsersVotesEntity::table_name(),
        );
        let res: (u64,) = sqlx::query_as(&sql)
            .bind(id)
            .bind("answer")
            .bind(username)
            .fetch_one(&self.mysql_pool)
            .await?;
        Ok(res.0 > 0)
    }

    // 取消回答点赞
    async fn cancel_vote(&self, id: u64, username: &str) -> anyhow::Result<bool> {
        let sql = format!(
            r#"delete from {} where target_id = ? and target_type = ? and created_by = ?"#,
            UsersVotesEntity::table_name(),
        );
        println!("cancel vote sql:{}", sql);

        let mysql_pool = &self.mysql_pool;
        let mut tx = mysql_pool.begin().await?;
        // 插入积分明细记录
        let affect_res = sqlx::query(&sql)
            .bind(id)
            .bind("answer")
            .bind(username)
            // 在sqlx 0.7版本以上，execute这里需要对tx进行解引用并获取内部DB的可变引用connection
            .execute(tx.deref_mut())
            .await?;
        info!("cancel vote affect_rows:{}", affect_res.rows_affected());

        // 先查询回答点赞数
        let sql = format!(
            "select id,agree_count from {} where id = ?",
            AnswersEntity::table_name(),
        );
        let res: (u64, u64) = sqlx::query_as(&sql).bind(id).fetch_one(mysql_pool).await?;
        let agree_count = res.1; // 当前回答点赞数
        let mut remain = agree_count as i64 - 1; // 取消点赞后的点赞数
        if remain <= 0 {
            info!(
                "current answer id:{} agree_count:{} remain:{}",
                id, agree_count, remain
            );
            remain = 0;
        }

        // 更新当前问题点赞数
        let sql = format!(
            r#"update {} set agree_count = ?,updated_at = ? where id = ?"#,
            AnswersEntity::table_name(),
        );
        let updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("cancel vote update sql:{}", sql);
        let affect_res = sqlx::query(&sql)
            .bind(remain)
            .bind(updated_at)
            .bind(id)
            .execute(tx.deref_mut())
            .await?;
        info!(
            "update answer vote affect_rows:{}",
            affect_res.rows_affected()
        );

        // 提交事务
        tx.commit().await?;
        Ok(true)
    }

    async fn handler_agree(&self, id: u64, username: &str, action: &str) -> anyhow::Result<bool> {
        // 判断是否点赞
        let res = self.has_voted(id, username).await;
        if action == "up" {
            if res.is_ok() {
                // 已经点赞，直接返回即可
                println!("user:{} has voted answer id:{}", username, id);
                return Ok(false);
            }

            return self.vote(id, username).await;
        }

        // 取消点赞处理逻辑
        if let Err(err) = res {
            let err = err.downcast().unwrap();
            match err {
                sqlx::Error::RowNotFound => {
                    // 未点赞
                    println!("user:{} does not vote answer id:{}", username, id);
                    Ok(false)
                }
                other => {
                    // 其他未知错误
                    println!("query user:{} answer vote error:{}", username, id);
                    Err(anyhow::Error::from(other))
                }
            }
        } else {
            self.cancel_vote(id, username).await
        }
    }
}

#[async_trait::async_trait]
impl VoteRepo for VoteRepoImpl {
    async fn publish(&self, msg: &VoteMessage) -> anyhow::Result<bool> {
        todo!()
    }

    async fn consumer(&self, target_type: &str) {
        todo!()
    }
}
