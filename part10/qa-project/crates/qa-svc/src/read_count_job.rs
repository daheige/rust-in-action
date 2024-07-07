use crate::config::{mysql, xpulsar, xredis, APP_CONFIG};
use crate::domain::repository::{ReadCountRepo, UserVoteRepo};
use infras::{job_graceful_shutdown, Logger}; // 日志模块

use crate::infrastructure::read_count;
use crate::infrastructure::read_count::new_read_count_repo;
use log::info;
use std::io::Write;
use std::process;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tokio::sync::RwLock;

mod config;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("hello,qa-read-count-job");
    // 如果想在启动时改变日志级别，可以通过指定环境变量启动应用
    // 启动方式：RUST_LOG=debug cargo run --bin qa-svc
    std::env::set_var("RUST_LOG", "debug");
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

    // create redis pool
    let redis_pool = xredis::pool(&APP_CONFIG.redis_conf).expect("redis pool init failed");
    let app_state = config::ReadCountJobAppState {
        // 这里等价于mysql_pool: mysql_pool
        // 当变量名字一样时，是可以直接用变量名字简写模式，是rust的语法糖
        mysql_pool,
        // 这里等价于pulsar_client: pulsar_client
        redis_pool,
    };

    let stop1 = stop.clone();
    let read_count_repo = new_read_count_repo(app_state.redis_pool, app_state.mysql_pool);
    // 处理问题阅读数
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        loop {
            let exit = stop1.read().await;
            if *exit {
                info!("recv shutdown signal,consumer will stop...");
                break;
            }

            interval.tick().await;
            let res = read_count_repo.handler("question").await;
            if let Err(err) = res {
                info!("handler read_count error:{}", err);
            }
        }
    });

    // 当接收到退出信号量时候，更新stop为true
    info!("recv data:{:?}", recv.recv().unwrap());
    let mut stop = stop.write().await;
    *stop = true;
    info!("read_count job shutdown success");
    Ok(())
}
