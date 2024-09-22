# 点赞job核心代码实现

```rust
use crate::config::{mysql, xpulsar, APP_CONFIG};
use crate::domain::repository::UserVoteRepo;
use crate::infrastructure::vote::new_vote_repo;
use infras::{graceful_shutdown, Logger}; // 日志模块

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
// 日志level 优先级  error > warn > info > debug > trace
// 启动方式：RUST_LOG=debug cargo run --bin qa-vote-job
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("hello,qa-vote-job");
// std::env::set_var("RUST_LOG", "debug");
    Logger::new().init(); // 使用默认方式初始化日志配置
    println!("current process pid:{}", process::id());

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

    // 平滑退出stop标识，用于消费者退出标识
    // 它是一个引用计数bool类型的异步读写锁
    let stop = Arc::new(RwLock::new(false));
    let stop1 = stop.clone();
    let vote_repo = new_vote_repo(app_state.mysql_pool, app_state.pulsar_client);
    // 通过tokio::spawn异步执行消息实时消费
    tokio::spawn(async move {
        println!("run answer vote job...");
        // 回答点赞消息处理
        let _ = vote_repo.consumer("answer", stop1).await;
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

    println!("vote job shutdown success");
    Ok(())
}

```

1. 从上述代码中看出，在main函数中，首先创建了mysql pool连接池和pulsar client客户端句柄。
2. 然后，通过new_vote_repo函数创建了一个UserVoteRepo实例对象vote_repo。
3. 接着，通过tokio::spawn创建了一个异步任务，用于消费回答点赞和取消点赞所对应的pulsar消息。
4. 在异步任务async块中，我将stop1传入consumer方法中，并使用await关键字来驱动这个异步方法执行。
5. 最后，通过tokio::spawn创建了一个异步任务运行平滑退出graceful_shutdown函数，它的作用是等待退出信号量到来，实现程序平滑退出。