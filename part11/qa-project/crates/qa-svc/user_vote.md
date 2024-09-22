# UserVoteRepo trait具体实现

UserVoteRepo trait中的publish和consumer方法具体实现所对应的代码片段如下所示：

```rust
// qa-svc/src/infrastructure/vote/user_vote.rs 文件
use crate::domain::entity::{AnswersEntity, UsersVotesEntity, VoteMessage};
use crate::domain::repository::UserVoteRepo;
use chrono::Local;
use futures::TryStreamExt;
use infras::sql_utils::gen_in_placeholder;
use log::{error, info};
use pulsar::{producer, proto, Consumer, Pulsar, SubType, TokioExecutor};
use std::collections::HashMap;
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

    // 当前用户是否对某个实体对象已点赞
    async fn has_voted(
        &self,
        target_id: u64,
        target_type: &str,
        created_by: &str,
    ) -> anyhow::Result<bool> {
        let sql = format!(
            "select id from {} where target_id = ? and target_type = ? and created_by = ?",
            UsersVotesEntity::table_name(),
        );
        let res: (u64, ) = sqlx::query_as(&sql)
            .bind(target_id)
            .bind(target_type)
            .bind(created_by)
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
        target_id: u64,
        created_by: &str,
        action: &str,
    ) -> anyhow::Result<bool> {
        let res = self.is_voted(target_id, "answer", created_by).await;
        if action == "up" {
            if res.is_ok() {
                // 已经点赞，直接返回即可
                info!("user:{} has voted answer id:{}", created_by, target_id);
                return Ok(false);
            }

            // 执行点赞操作
            self.answer_vote(target_id, created_by).await
        } else {
            // 取消点赞
            let has_voted = res.unwrap_or(false);
            if !has_voted {
                // 未点赞
                info!("user:{} does not vote answer id:{}", created_by, target_id);
                return Ok(false);
            }

            self.cancel_answer_vote(target_id, created_by).await
        }
    }
}

#[async_trait::async_trait]
impl UserVoteRepo for UserVoteRepoImpl {
    async fn is_voted(
        &self,
        target_id: u64,
        target_type: &str,
        username: &str,
    ) -> anyhow::Result<bool> {
        self.has_voted(target_id, target_type, username).await
    }

    async fn is_batch_voted(
        &self,
        target_ids: &Vec<u64>,
        target_type: &str,
        username: &str,
    ) -> anyhow::Result<HashMap<u64, bool>> {
        // 将参数转换为(?,?)格式
        let parameters = gen_in_placeholder(target_ids.len());

        let sql = format!(
            r#"
                select target_id from {} where target_id in ({})
                and target_type = ? and created_by = ?
            "#,
            UsersVotesEntity::table_name(),
            parameters,
        );

        println!("exec batch voted sql:{}", sql);
        let mut query = sqlx::query_as(&sql);
        // 绑定参数
        for id in target_ids {
            query = query.bind(id);
        }

        // 将查询结果集映射到Vec中
        let records: Vec<(u64, )> = query
            .bind(target_type)
            .bind(username)
            .fetch_all(&self.mysql_pool)
            .await?;
        let mut m = HashMap::with_capacity(records.len() + 1);
        for item in records {
            m.insert(item.0, true);
        }

        Ok(m)
    }

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
                break;
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

            // 消费消息逻辑,这里需要将用户点赞明细落地到数据库DB中，并更新实体点赞数
            info!("got message data:{:?}", data);

            if target_type == "answer" {
                // 处理回答点赞和取消点赞操作
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
            } else {
                // 其他实体的点赞，可以在这里添加业务逻辑处理
                // ...
                info!(
                    "target_id:{} target_type:{} created_by:{} action:{}",
                    data.target_id, data.target_type, data.created_by, data.action
                )
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
```

- 从上述代码可以看出，在publish方法中，首先调用pulsar_client.producer方法创建了一个producer对象，同时在上面设置了topic名字为user-vote-topic以及pulsar其他配置。
- 然后，将点赞和取消点赞的消息msg(VoteMessage)发送到该topic中。

consumer方法中的业务逻辑如下：

- 在consumer方法中，首先调用pulsar_client.consumer方法创建了一个consumer对象，同时设置了consumer消费组名字为group-1。
- 然后，使用while let
  Some模式匹配从consumer对象调用try_next方法从pulsar消息队列中读取消息msg。在while循环中，通过match模式匹配将msg消息反序列化为data(
  VoteMessage)。
- 接着，根据target_type实体类型调用不同的方法，实现消息点赞和取消点赞。
- 最后，调用consumer.ack方法向puslar消息队列所对应的节点发送ack通知，表示该消息已经被消费。
