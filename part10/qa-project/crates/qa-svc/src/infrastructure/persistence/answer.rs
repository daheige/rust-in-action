use crate::domain::entity::{
    AnswerListReply, AnswersEntity, CountInfo, QuestionsEntity, UsersEntity, UsersVotesEntity,
};
use crate::domain::repository::AnswerRepo;
use axum::routing::any;
use chrono::{Local, NaiveDateTime};
use log::info;
use std::ops::DerefMut;

struct AnswerRepoImpl {
    mysql_pool: sqlx::MySqlPool,
}

pub fn new_answer_repo(mysql_pool: sqlx::MySqlPool) -> impl AnswerRepo {
    AnswerRepoImpl { mysql_pool }
}

impl AnswerRepoImpl {
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
}

#[async_trait::async_trait]
impl AnswerRepo for AnswerRepoImpl {
    async fn add(&self, answer: &AnswersEntity) -> anyhow::Result<u64> {
        let sql = format!(
            "insert into {} (question_id,content,created_by,created_at) value(?,?,?,?)",
            AnswersEntity::table_name()
        );

        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let affect_rows = sqlx::query(&sql)
            .bind(&answer.question_id)
            .bind(&answer.content)
            .bind(&answer.created_by)
            .bind(created_at)
            .execute(&self.mysql_pool)
            .await?;

        let id = affect_rows.last_insert_id();
        println!("current insert answer id = {}", id);

        Ok(id)
    }

    async fn delete(&self, id: u64, username: &str) -> anyhow::Result<()> {
        let sql = format!(
            "delete from {} where id = ? and created_by = ?",
            AnswersEntity::table_name()
        );
        let affect_rows = sqlx::query(&sql)
            .bind(id)
            .bind(username)
            .execute(&self.mysql_pool)
            .await?;
        println!("affected rows: {}", affect_rows.rows_affected());

        Ok(())
    }

    async fn update(&self, id: u64, content: &str, updated_by: &str) -> anyhow::Result<()> {
        let sql = format!(
            r#"update {} set content = ?,updated_by = ?,updated_at = ? where id = ?"#,
            AnswersEntity::table_name(),
        );

        println!("update sql:{}", sql);
        let updated_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let affect_res = sqlx::query(&sql)
            .bind(content)
            .bind(updated_by)
            .bind(updated_at)
            .bind(id)
            .execute(&self.mysql_pool)
            .await?;
        println!(
            "current answer affected_rows = {}",
            affect_res.rows_affected()
        );
        Ok(())
    }

    async fn fetch_one(&self, id: u64) -> anyhow::Result<AnswersEntity> {
        let sql = format!(
            "select * from {} where id = ? limit 1",
            AnswersEntity::table_name()
        );

        // query_as将其映射到结构体中
        let answer: AnswersEntity = sqlx::query_as(&sql)
            .bind(id)
            .fetch_one(&self.mysql_pool)
            .await?;
        Ok(answer)
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

    async fn lists(
        &self,
        question_id: u64,
        page: u64,
        limit: u64,
        order_by: &str,
    ) -> anyhow::Result<AnswerListReply> {
        let sql = format!(
            "select count(*) as cnt from {} where question_id = ?",
            AnswersEntity::table_name(),
        );
        let res: CountInfo = sqlx::query_as(&sql)
            .bind(question_id)
            .fetch_one(&self.mysql_pool)
            .await?;
        // mysql对应的bigint类型，对应rust的i64，这里使用as将其转换为u64类型
        let total = res.cnt as u64;
        if total == 0 {
            let reply = AnswerListReply::new(vec![], total, page, limit);
            return Ok(reply);
        }

        let sql = format!(
            "select * from {} where question_id = ? order by {} limit {} offset {}",
            AnswersEntity::table_name(),
            order_by,
            limit,
            (page - 1) * limit,
        );
        // query_as将其映射到vec中
        let answers: Vec<AnswersEntity> = sqlx::query_as(&sql)
            .bind(question_id)
            .fetch_all(&self.mysql_pool)
            .await?;
        let reply = AnswerListReply::new(answers, total, page, limit);
        Ok(reply)
    }
}
