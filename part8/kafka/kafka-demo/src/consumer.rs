use env_logger;
// 引入kafka包
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

use log::{error, info};
// use std::env;
use std::thread;
use std::time::Duration;
// 命令终端运行方式：RUST_LOG=debug cargo run --bin kafka-demo-consumer
fn main() {
    // 初始化logger配置
    // 日志level 优先级  error > warn > info > debug > trace
    // 设置日志级别环境变量，这里注释掉了，启动的时可手动指定RUST_LOG=debug
    // env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // 消费消息
    let broker = "localhost:9092";
    let topic = "my-topic";
    let group = "my-group";
    info!("consumer message begin...");
    let res = consumer_message(group, topic, vec![broker.to_string()]);
    if let Err(err) = res {
        error!("consumer message err:{}", err);
    }
}

// 消费消息
// 函数返回Result<(), KafkaError>
fn consumer_message(group: &str, topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    // 创建consumer connection
    let mut conn = Consumer::from_hosts(brokers)
        .with_topic(topic.to_string()) // 消息主题
        .with_group(group.to_string()) // 设置分组
        .with_fallback_offset(FetchOffset::Earliest) // 设置offset
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()?;
    loop {
        let message_sets = conn.poll()?;
        if message_sets.is_empty() {
            info!("no message available right now");
            thread::sleep(Duration::from_secs(2)); // 当没有消息的时候停顿2s
            continue;
        }

        // 为了方便查看value，我这里将value转换为String格式
        for ms in message_sets.iter() {
            let topic = ms.topic(); // 消息topic
            let partition = ms.partition(); // 消息topic对应的partition
            for m in ms.messages() {
                info!(
                    "received message topic:{} group:{} partition:{}@offset:{}: value:{}",
                    topic,
                    group,
                    partition,
                    m.offset,
                    String::from_utf8(m.value.to_vec()).unwrap_or("".to_string()),
                );
            }

            if let Err(err) = conn.consume_messageset(ms) {
                error!(
                    "consume message topic:{} group:{} error:{}",
                    topic, group, err
                );
            } else {
                info!("consume message topic:{} group:{} success", topic, group)
            }
        }

        // commit consumed
        conn.commit_consumed()?;
    }
}
