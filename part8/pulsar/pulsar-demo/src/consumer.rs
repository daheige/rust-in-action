// 定义xpulsar模块
mod xpulsar;

// 引入相关包
use futures::TryStreamExt;
use log::{error, info};
use pulsar::{Consumer, Error as PulsarError, SubType};
use std::env;
use xpulsar::Message;

#[tokio::main]
async fn main() -> Result<(), PulsarError> {
    println!("consumer pulsar message...");
    env::set_var("RUST_LOG", "debug");
    env_logger::init(); // 操作日志初始化
    
    // pulsar连接地址
    let addr = env::var("PULSAR_ADDRESS").unwrap_or("pulsar://127.0.0.1:6650".to_string());
    let pulsar_client = xpulsar::PulsarConf::new(&addr)
        .client()
        .await
        .expect("create pulsar client failed");

    let topic = "my-topic";
    // 创建consumer
    let mut consumer: Consumer<Message, _> = pulsar_client
        .consumer()
        .with_topic(topic) // 设置topic
        .with_consumer_name("group-1") // 设置消费组名字
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("my_topic test") // 订阅的名字
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
        println!("got message data:{}", data.payload.as_str());

        // 提交消息ack确认
        consumer.ack(&msg).await?;
        counter += 1;
        info!("got message count:{}", counter);
    }

    Ok(())
}
