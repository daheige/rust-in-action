use crate::config::{mysql, xpulsar, APP_CONFIG};
use crate::domain::repository::UserVoteRepo;
use crate::infrastructure::vote::new_vote_repo;
use infras::{job_graceful_shutdown, Logger}; // 日志模块

use std::io::Write;
use std::process;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tokio::sync::RwLock;

mod config;
mod domain;
mod infrastructure;

// 如果想在启动时改变日志级别，可以通过指定环境变量启动应用
// 启动方式：RUST_LOG=debug cargo run --bin qa-vote-job
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("hello,qa-vote-job");
    // std::env::set_var("RUST_LOG", "debug");
    Logger::new().init(); // 使用默认方式初始化日志配置
    println!("current process pid:{}", process::id());
    // 平滑退出，stop用于消费者退出标识，它是一个引用计数且持有读写锁的bool类型的共享变量
    let stop = Arc::new(RwLock::new(false));
    let (send, recv) = mpsc::channel();
    tokio::spawn(async move {
        job_graceful_shutdown(Duration::from_secs(APP_CONFIG.graceful_wait_time), send).await;
    });

    // create mysql pool
    let mysql_pool = mysql::pool(&APP_CONFIG.mysql_conf)
        .await
        .expect("mysql pool init failed");

    // init pulsar client
    let pulsar_client = xpulsar::client(&APP_CONFIG.pulsar_conf)
        .await
        .expect("pulsar client init failed");

    let app_state = config::VoteJobAppState {
        // 这里等价于mysql_pool: mysql_pool
        // 当变量名字一样时，是可以直接用变量名字简写模式，是rust的语法糖
        mysql_pool,
        // 这里等价于pulsar_client: pulsar_client
        pulsar_client,
    };

    let stop1 = stop.clone();
    let vote_repo = new_vote_repo(app_state.mysql_pool.clone(), app_state.pulsar_client);

    // 回答点赞消息处理
    // 通过tokio::spawn异步执行消息实时消费
    tokio::spawn(async move {
        let _ = vote_repo.consumer("answer", stop1).await;
    });

    // 当接收到退出信号量时候，更新stop为true
    println!("recv data:{:?}", recv.recv().unwrap());
    let mut stop = stop.write().await;
    *stop = true;
    println!("shutdown success");
    Ok(())
}
