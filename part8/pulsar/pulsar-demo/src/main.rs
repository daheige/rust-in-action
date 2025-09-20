// 定义xpulsar模块,主要负责puslar client创建和消息Message序列化处理
mod xpulsar;

// 引入相关包
use log::info;
use pulsar::{Error as PulsarError, producer, proto};
use std::env;
use xpulsar::{Message, PulsarConf}; // 引入xpulsar模块中的Message和PulsarConf

#[tokio::main]
async fn main() -> Result<(), PulsarError> {
    println!("publish pulsar message");
    // unsafe {
    //     env::set_var("RUST_LOG", "debug");
    // }

    env_logger::init(); // 初始化操作日志配置

    // pulsar连接地址
    let addr = env::var("PULSAR_ADDRESS").unwrap_or("pulsar://127.0.0.1:6650".to_string());
    // 初始化pulsar client客户端
    let pulsar_client = PulsarConf::new(&addr)
        .client()
        .await
        .expect("create pulsar client failed");

    let topic = "my-topic";
    // 创建producer
    let mut producer = pulsar_client
        .producer()
        .with_topic(topic)
        .with_name("my_producer")
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

    // 消息发送
    let mut counter: usize = 0;
    loop {
        let msg = Message {
            payload: format!("hello: {}", counter),
        };
        info!("sent msg:{:?}", msg);
        // 发送消息
        producer.send_non_blocking(msg).await?;

        counter += 1;
        info!("publish message count:{}", counter);
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        if counter >= 5 {
            // 这里仅发送5条消息
            break;
        }
    }

    Ok(())
}
