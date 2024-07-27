// 自定义模块
mod config;
mod handlers;
mod middleware;
mod routers;
mod utils;

// 引入Config
use config::{AppState, APP_CONFIG};
use infras::{graceful_shutdown, prometheus_init, Logger};

// serde序列化处理
use log::info;
use pb::qa::qa_service_client::QaServiceClient;
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // 如果想在启动时改变日志级别，可以通过指定环境变量启动应用
    // 日志level 优先级  error > warn > info > debug > trace
    // 启动方式：RUST_LOG=debug cargo run --bin gateway
    // std::env::set_var("RUST_LOG", "debug");
    Logger::new().init(); // 使用默认方式初始化日志配置

    info!("qa gateway start...");

    // 开发过程中，可以取消下面的注释
    if APP_CONFIG.app_debug {
        println!("conf:{:?}", APP_CONFIG);
    }

    // build http /metrics endpoint
    let metrics_server = prometheus_init(APP_CONFIG.metrics_port);
    let metrics_handler = tokio::spawn(metrics_server);

    // http gateway handler
    let gateway_handler = tokio::spawn(async move {
        // Create grpc client
        let grpc_client = QaServiceClient::connect(APP_CONFIG.grpc_address.as_str())
            .await
            .expect("failed to connect grpc service");

        // 通过arc引用计数的方式传递state
        let app_state = Arc::new(AppState { grpc_client });

        let address: SocketAddr = format!("0.0.0.0:{}", APP_CONFIG.app_port).parse().unwrap();
        println!("current process pid:{}", process::id());
        println!("http gateway run on:{}", address.to_string());

        // Create axum router
        let router = routers::router::api_router(app_state);

        // Create a `TcpListener` using tokio.
        let listener = TcpListener::bind(address).await.unwrap();

        // Run the server with graceful shutdown
        axum::serve(listener, router)
            .with_graceful_shutdown(graceful_shutdown(Duration::from_secs(
                APP_CONFIG.graceful_wait_time,
            )))
            .await
            .expect("failed to start gateway service");
    });

    // start http gateway and metrics service
    let _ = tokio::try_join!(gateway_handler, metrics_handler)
        .expect("failed to start http gateway and metrics service");
}
