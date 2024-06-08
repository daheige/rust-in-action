mod config;
mod entity;
mod services;

// 引入模块
use crate::config::{mysql, xpulsar, APP_CONFIG};
use pb::qa::qa_service_server::QaServiceServer;
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;

use tonic::transport::Server;

// 这个file descriptor文件是build.rs中定义的descriptor_path路径
// 读取proto file descriptor bin二进制文件
pub(crate) const PROTO_FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("../rpc_descriptor.bin");

/// 采用 tokio 运行时来执行grpc server
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, qa-svc!");
    // env::set_var("RUST_LOG", "debug");
    env_logger::init(); // 初始化操作日志配置

    println!("app_debug:{:?}", APP_CONFIG.app_debug);
    println!("current process pid:{}", process::id());

    // 微服务启动地址
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

    // grpc reflection服务
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PROTO_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // 启动grpc service
    let qa_service = services::QAServiceImpl::new();
    Server::builder()
        .add_service(reflection_service)
        .add_service(QaServiceServer::new(qa_service))
        .serve(address)
        .await?;

    Ok(())
}