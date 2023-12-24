// 引入axum包
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;

// define module
mod handlers;

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:8080";
    println!("server run on:{}", address);
    println!("server pid:{}", process::id()); // 服务启动的进程id

    // 为了模拟存储，这里采用hash map结构存储短链接字符串和原始地址映射关系
    let shared_state = Arc::new(handlers::SharedState::default());

    // 创建axum router
    let router = Router::new()
        .route("/:key", get(handlers::short_url)) // eg:your_domain/43KClC格式的短链请求
        .route(
            "/create",
            post(handlers::create),
        ) // 创建短链接url
        .with_state(Arc::clone(&shared_state))
        .fallback(handlers::not_found_handler);

    // 创建axum app实例并启动服务
    let addr: SocketAddr = address.parse().unwrap();
    axum::Server::bind(&addr)
        .serve(router.into_make_service()) // 将router转化为service
        .with_graceful_shutdown(handlers::graceful_shutdown()) // 设置平滑退出函数
        .await
        .unwrap();
}
