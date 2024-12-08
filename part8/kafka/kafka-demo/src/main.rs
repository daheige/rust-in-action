use env_logger;
// 引入kafka包
use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};
use log::{error, info};
use std::time::Duration;
// use std::{env, thread};

// 命令终端运行方式：RUST_LOG=debug cargo run kafka-demo
// 这种运行方式，就会把对应的操作日志输出到终端
fn main() {
    // 初始化logger配置
    // 日志level 优先级  error > warn > info > debug > trace
    // 设置日志级别环境变量，这里注释掉了，启动的时可手动指定RUST_LOG=debug
    // env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let broker = "localhost:9092"; // kafka broker
    let topic = "my-topic"; // kafka topic
    println!("publish message begin");
    let mut i = 0;
    // 发送消息到kafka topic中
    // 这里发送5个消息到my-topic中
    while i < 5 {
        info!("current index:{}", i);
        let msg = format!("hello world: {}", i);
        info!("current msg:{}", msg);
        if let Err(e) = publish_message(msg.as_bytes(), topic, vec![broker.to_string()]) {
            error!("failed producing message error:{}", e);
        } else {
            info!("current index:{} producing message success", i);
        }

        i += 1;
    }
    println!("publish message end")
}

// 发送消息
// 函数返回Result<(), KafkaError>
fn publish_message(data: &[u8], topic: &str, brokers: Vec<String>) -> Result<(), KafkaError> {
    info!("publish message at {:?} to {}", brokers, topic);

    // 创建kafka producer
    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1)) // 设置ack timeout
        .with_required_acks(RequiredAcks::One) // 消息需要确认
        .create()?;

    // 使用producer发送消息
    producer.send(&Record {
        topic, // 主题
        partition: -1,
        key: (),
        value: data, // 消息
    })?;

    // 你也可以通过下面的方式发送消息
    // producer.send(&Record::from_value(topic, data))?;

    Ok(())
}
