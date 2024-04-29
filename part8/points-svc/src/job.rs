use crate::config::{mysql, xpulsar, APP_CONFIG};
use crate::entity::PointsMessage;
use futures::TryStreamExt;
use log::{error, info};
use pulsar::{Consumer, SubType};
use std::process;
use std::sync::Arc;

// 定义项目相关的模块
mod config; // 用于mysql和redis config初始化和连接池管理
mod entity; // 实体对象定义
mod handlers; // 用于http handler处理
mod infras; // 项目中基础设施层封装

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // env::set_var("RUST_LOG", "debug");
    env_logger::init(); // 初始化操作日志配置
    println!("points-svc job");
    println!("app_debug:{:?}", APP_CONFIG.app_debug);
    println!("current process pid:{}", process::id());
    // mysql pool
    let mysql_pool = mysql::pool(&APP_CONFIG.mysql_conf)
        .await
        .expect("mysql pool init failed");
    let pulsar_client = xpulsar::client(&APP_CONFIG.pulsar_conf)
        .await
        .expect("pulsar client init failed");

    // 通过arc引用计数的方式传递state
    let app_state = Arc::new(config::AppState {
        mysql_pool, // 这里等价于mysql_pool: mysql_pool,当变量名字一样时，是可以直接用变量名字简写模式，是rust的语法糖
        pulsar_client, // 这里等价于pulsar_client: pulsar_client
    });

    let topic = "points-topic";
    // 创建consumer
    let mut consumer: Consumer<PointsMessage, _> = app_state
        .pulsar_client
        .consumer()
        .with_topic(topic) // 设置topic
        .with_consumer_name("group-1") // 设置消费组名字
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("points-sys") // 订阅的名字
        .build()
        .await?;

    info!("consumer has run...");
    let mut counter: usize = 0;
    // 接收消息并消费
    while let Some(msg) = consumer.try_next().await? {
        // println!("metadata:{:?}", msg.message_id());
        // println!("id:{:?}", msg.message_id());
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(err) => {
                error!("could not deserialize message:{:?}", err);
                continue;
            }
        };

        // 消费消息逻辑
        // 这里需要处理用户积分明细落地到数据库DB中，并更新用户的积分总数 todo
        println!("got message data:{:?}", data);

        // 提交消息ack确认
        consumer.ack(&msg).await?;
        counter += 1;
        info!("got message count:{}", counter);
    }

    Ok(())
}
