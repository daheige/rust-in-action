use crate::domain::entity::{AnswersEntity, UsersVotesEntity, VoteMessage};
use crate::domain::repository::UserVoteRepo;
use chrono::Local;
use futures::TryStreamExt;
use log::{error, info};
use pulsar::{producer, proto, Consumer, Pulsar, SubType, TokioExecutor};
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::RwLock;

struct UserVoteRepoImpl {
    mysql_pool: sqlx::MySqlPool,
    pulsar_client: Pulsar<TokioExecutor>,
}

pub fn new_vote_repo(
    mysql_pool: sqlx::MySqlPool,
    pulsar_client: Pulsar<TokioExecutor>,
) -> impl UserVoteRepo {
    UserVoteRepoImpl {
        mysql_pool,
        pulsar_client,
    }
}

impl UserVoteRepoImpl {
    // 回答点赞
    async fn answer_vote(&self, id: u64, username: &str) -> anyhow::Result<bool> {
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let sql = format!(
            r#"insert into {} (target_id,target_type,created_by,created_at) value (?,?,?,?)"#,
            UsersVotesEntity::table_name(),
        );
        println!("insert vote sql:{}", sql);

        let mysql_pool = &self.mysql_pool;
        let mut tx = mysql_pool.begin().await?;
        // 插入点赞明细记录
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

    // 当前用户是否对某个回答已点赞
    async fn has_answer_voted(&self, id: u64, username: &str) -> anyhow::Result<bool> {
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
    async fn cancel_answer_vote(&self, id: u64, username: &str) -> anyhow::Result<bool> {
        // 删除点赞记录
        let sql = format!(
            r#"delete from {} where target_id = ? and target_type = ? and created_by = ?"#,
            UsersVotesEntity::table_name(),
        );
        println!("cancel vote sql:{}", sql);
        let mysql_pool = &self.mysql_pool;
        let mut tx = mysql_pool.begin().await?;
        // 删除点赞明细记录
        let affect_res = sqlx::query(&sql)
            .bind(id)
            .bind("answer")
            .bind(username)
            // 在sqlx 0.7版本以上，execute这里需要对tx进行解引用并获取内部DB的可变引用connection
            .execute(tx.deref_mut())
            .await?;
        info!("cancel vote affect_rows:{}", affect_res.rows_affected());

        // 查询回答点赞数
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

    async fn handler_answer_agree(
        &self,
        id: u64,
        username: &str,
        action: &str,
    ) -> anyhow::Result<bool> {
        // 判断是否点赞
        let res = self.has_answer_voted(id, username).await;
        if action == "up" {
            if res.is_ok() {
                // 已经点赞，直接返回即可
                println!("user:{} has voted answer id:{}", username, id);
                return Ok(false);
            }

            return self.answer_vote(id, username).await;
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
            self.cancel_answer_vote(id, username).await
        }
    }
}

#[async_trait::async_trait]
impl UserVoteRepo for UserVoteRepoImpl {
    // 发送用户点赞消息
    async fn publish(&self, message: VoteMessage) -> anyhow::Result<bool> {
        // 消息主题topic
        let topic = "user-vote-topic";

        // 创建producer
        let mut producer = self
            .pulsar_client
            .producer()
            .with_topic(topic)
            .with_name("qa-sys")
            .with_options(producer::ProducerOptions {
                schema: Some(proto::Schema {
                    r#type: proto::schema::Type::String as i32,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .build()
            .await?;

        // 验证producer connection是否生效
        producer.check_connection().await?;

        // 将消息推送到Pulsar mq队列对应的topic主题中
        // producer.send(message).await?;
        producer.send_non_blocking(message).await?;

        Ok(true)
    }

    // 消费点赞消息，并将数据持久化存储到mysql数据库对应的表中
    async fn consumer(&self, target_type: &str, stop: Arc<RwLock<bool>>) -> anyhow::Result<()> {
        let topic = "user-vote-topic";

        // 创建consumer
        let mut consumer: Consumer<VoteMessage, _> = self
            .pulsar_client
            .consumer()
            .with_topic(topic) // 设置topic
            .with_consumer_name("group-1") // 设置消费组名字
            .with_subscription_type(SubType::Exclusive)
            .with_subscription("qa-sys") // 订阅的名字
            .build()
            .await?;
        info!("consumer user vote message begin...");
        let mut counter: usize = 0;
        // 接收消息并消费
        while let Some(msg) = consumer.try_next().await? {
            let exit = stop.read().await;
            if *exit {
                info!("recv shutdown signal,consumer will stop...");
                return Ok(());
            }

            // 输出消息id
            info!("message id:{:?}", msg.message_id());
            // 解析message到VoteMessage结构体中
            let data = match msg.deserialize() {
                Ok(data) => data,
                Err(err) => {
                    error!("could not deserialize message:{:?}", err);
                    continue;
                }
            };

            // 消费消息逻辑,这里需要将用户点赞明细落地到数据库DB中，并更新回答点赞数
            info!("got message data:{:?}", data);

            if target_type == "answer" {
                let reply = self
                    .handler_answer_agree(data.target_id, &data.created_by, &data.action)
                    .await;
                if let Err(err) = reply {
                    error!(
                        "message target_id:{} target_type:{} created_by:{} action:{} error:{}",
                        data.target_id, data.target_type, data.created_by, data.action, err,
                    );
                    continue;
                }
            }

            // 提交消息ack确认
            consumer.ack(&msg).await?;
            info!(
                "consumer message target_id:{} target_type:{} action:{} success",
                data.target_id, data.target_type, data.action,
            );

            counter += 1;
            info!("got message count:{}", counter);
        }

        Ok(())
    }
}
