// 引入axum包
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;

// 定义处理器函数对应的模块
mod handlers;

#[tokio::main]
async fn main() {
    // HTTP服务运行地址
    let address = "127.0.0.1:8080";
    println!("server run on:{}", address);
    println!("server pid:{}", process::id()); // 服务启动的进程id

    // with_state 共享数据
    let shared_state = Arc::new(handlers::AppState::default());
    // 创建axum router
    let router = Router::new()
        .route("/:key", get(handlers::short_url))
        .route("/create-short-url", post(handlers::create_short_url))
        .with_state(shared_state) // 通过with_state方式传递共享数据shared_state
        .fallback(handlers::not_found_handler);

    // 创建一个listener对象
    let addr: SocketAddr = address.parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // 启动HTTP服务
    axum::serve(listener, router)
        .with_graceful_shutdown(handlers::graceful_shutdown()) // 设置平滑退出函数
        .await
        .unwrap();
}
