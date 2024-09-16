mod application;
mod config;
mod domain;
mod infrastructure;

// 引入模块
use crate::config::{mysql, xpulsar, xredis, APP_CONFIG};
use infras::{graceful_shutdown, prometheus_init, Logger};
use log::info;
use pb::qa::qa_service_server::QaServiceServer;
use std::net::SocketAddr;
use std::process;
use std::time::Duration;
use tonic::transport::Server;

// 这个file descriptor文件是build.rs中定义的descriptor_path路径
// 读取proto file descriptor bin二进制文件
pub(crate) const PROTO_FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("../rpc_descriptor.bin");

// 如果想在启动时改变日志级别，可以通过指定环境变量启动应用
// 日志level 优先级  error > warn > info > debug > trace
// 启动方式：RUST_LOG=debug cargo run --bin qa-svc
// 采用 tokio 运行时来执行grpc server
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, qa-svc");
    // std::env::set_var("RUST_LOG", "debug");
    Logger::new().init(); // 使用默认方式初始化日志配置
    // 自定义方式初始化日志配置
    // Logger::new().with_custom().init();

    info!("app_debug:{:?}", APP_CONFIG.app_debug);
    info!("current process pid:{}", process::id());

    // 微服务启动地址
    let address: SocketAddr = format!("0.0.0.0:{}", APP_CONFIG.app_port).parse().unwrap();
    println!("app run on:{}", address.to_string());

    // mysql pool
    let mysql_pool = mysql::pool(&APP_CONFIG.mysql_conf)
        .await
        .expect("mysql pool init failed");
    // init pulsar client
    let pulsar_client = xpulsar::client(&APP_CONFIG.pulsar_conf)
        .await
        .expect("pulsar client init failed");
    let redis_pool = xredis::pool(&APP_CONFIG.redis_conf).expect("redis pool init failed");
    let app_state = config::AppState {
        // 这里等价于mysql_pool: mysql_pool
        // 当变量名字一样时，是可以直接用变量名字简写模式，是rust的语法糖
        mysql_pool,
        // 这里等价于pulsar_client: pulsar_client
        pulsar_client,
        redis_pool,
    };

    // grpc reflection服务
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PROTO_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // Create grpc service
    let qa_service = application::new_qa_service(app_state);
    let grpc_server = Server::builder()
        .add_service(reflection_service)
        .add_service(QaServiceServer::new(qa_service))
        .serve_with_shutdown(
            address,
            graceful_shutdown(Duration::from_secs(APP_CONFIG.graceful_wait_time)),
        );

    // build http /metrics endpoint
    let metrics_server = prometheus_init(APP_CONFIG.metrics_port);

    // create handler for each server
    let grpc_handler = tokio::spawn(grpc_server);
    let http_handler = tokio::spawn(metrics_server);
    let _ = tokio::try_join!(grpc_handler, http_handler)
        .expect("failed to start grpc service and metrics service");

    Ok(())
}
