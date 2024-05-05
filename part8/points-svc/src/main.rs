// 定义项目相关的模块
mod config; // 用于配置文件读取以及mysql和pulsar初始化
mod entity; // 实体对象定义
mod handlers; // 用于http handler处理
mod infras; // 项目中基础设施层封装
mod routers; // axum http框架路由模块

// 引入模块
use crate::config::{mysql, xpulsar, APP_CONFIG};
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // env::set_var("RUST_LOG", "debug");
    env_logger::init(); // 初始化操作日志配置

    println!("app_debug:{:?}", APP_CONFIG.app_debug);
    println!("current process pid:{}", process::id());

    let address: SocketAddr = format!("0.0.0.0:{}", APP_CONFIG.app_port).parse().unwrap();
    println!("app run on:{}", address.to_string());

    // mysql pool
    let mysql_pool = mysql::pool(&APP_CONFIG.mysql_conf)
        .await
        .expect("mysql pool init failed");
    let pulsar_client = xpulsar::client(&APP_CONFIG.pulsar_conf)
        .await
        .expect("pulsar client init failed");

    // 通过arc引用计数的方式传递state
    let app_state = Arc::new(config::AppState {
        // 这里等价于mysql_pool: mysql_pool,当变量名字一样时，是可以直接用变量名字简写模式，是rust的语法糖
        mysql_pool,
        // 这里等价于pulsar_client: pulsar_client
        pulsar_client,
    });

    // Create axum router
    let router = routers::api_router(app_state);

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(address).await?.into();

    // Run the server with graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(graceful_shutdown())
        .await?;
    Ok(())
}

// 平滑退出信号量处理
async fn graceful_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c =>{
            println!("received ctrl_c signal,server will exist...");
            tokio::time::sleep(Duration::from_secs(APP_CONFIG.graceful_wait_time)).await;
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
            tokio::time::sleep(Duration::from_secs(APP_CONFIG.graceful_wait_time)).await;
        },
    }

    println!("signal received,starting graceful shutdown");
}
