use crate::config::{mysql, xpulsar, xredis, APP_CONFIG};
use crate::domain::repository::{ReadCountRepo, UserVoteRepo};
use infras::{graceful_shutdown, Logger}; // 日志模块

// 引入实体阅读数对应的模块new_read_count_repo
use crate::infrastructure::read_count::new_read_count_repo;
use log::info;
use std::io::Write;
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

// 定义模块
mod config;
mod domain;
mod infrastructure;

// 如果想在启动时改变日志级别，可以通过指定环境变量启动应用
// 日志level 优先级  error > warn > info > debug > trace
// 启动方式：RUST_LOG=debug cargo run --bin qa-read_count-job
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("hello,qa-read-count-job");
    // std::env::set_var("RUST_LOG", "debug");
    Logger::new().init(); // 使用默认方式初始化日志配置
    println!("current process pid:{}", process::id());

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

    // 平滑退出stop标识，用于消费者退出标识，
    // 它是一个引用计数bool类型的异步读写锁
    let stop = Arc::new(RwLock::new(false));
    let stop1 = stop.clone();
    let read_count_repo = new_read_count_repo(app_state.redis_pool, app_state.mysql_pool);
    println!("run read_count job...");
    // 处理问题阅读数
    tokio::spawn(async move {
        // 每隔2s执行一次
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        loop {
            let exit = stop1.read().await;
            if *exit {
                info!("recv shutdown signal,consumer will stop...");
                break;
            }

            interval.tick().await;
            // println!("handler question read_count");
            let res = read_count_repo.handler("question").await;
            if let Err(err) = res {
                info!("handler read_count error:{}", err);
            }
        }
    });

    // 等待退出信号量的到来
    let handler = tokio::spawn(async move {
        graceful_shutdown(Duration::from_secs(APP_CONFIG.graceful_wait_time)).await;
    });

    // 这里会阻塞，只有接收到退出信号量，才会执行退出操作
    handler.await.unwrap();

    // 当接收到退出信号量时，就将stop的值设置为true
    let mut exit = stop.write().await;
    *exit = true;
    println!("read_count job shutdown success");
    Ok(())
}
