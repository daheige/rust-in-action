// 自定义模块
mod config;
mod handlers;
mod middleware;
mod routers;
mod utils;

// 引入Config
use config::AppState;
use infras::{Config, ConfigTrait};

// serde序列化处理
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use std::time::Duration;

// 引入tokio
use pb::qa::qa_service_client::QaServiceClient;
use tokio::net::TcpListener;
use tokio::signal;

// AppConfig 项目配置信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct AppConfig {
    app_debug: bool,         // 是否开启调试模式
    app_port: u32,           // http gateway port
    grpc_address: String,    // grpc service运行端口
    graceful_wait_time: u64, // http service平滑退出等待时间，单位s
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 读取配置文件
    let mut c = Config::new("app-gw.yaml");
    c.load().expect("read file failed");

    // 将配置文件内容解析到结构体中
    let conf: AppConfig = serde_yaml::from_str(c.content()).unwrap();
    // 开发过程中，可以取消下面的注释
    if conf.app_debug {
        println!("conf:{:?}", conf);
    }

    // Create grpc client
    let grpc_client = QaServiceClient::connect(conf.grpc_address).await?;

    // 通过arc引用计数的方式传递state
    let app_state = Arc::new(AppState { grpc_client });

    let address: SocketAddr = format!("0.0.0.0:{}", conf.app_port).parse().unwrap();

    println!("current process pid:{}", process::id());
    println!("app run on:{}", address.to_string());

    // Create axum router
    let router = routers::router::api_router(app_state);

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(address).await.unwrap();

    // Run the server with graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(graceful_shutdown(conf.graceful_wait_time))
        .await
        .unwrap();

    Ok(())
}

// graceful shutdown
async fn graceful_shutdown(wait_secs: u64) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
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
            tokio::time::sleep(Duration::from_secs(wait_secs)).await;
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
            tokio::time::sleep(Duration::from_secs(wait_secs)).await;
        },
    }

    println!("signal received,starting graceful shutdown");
}
