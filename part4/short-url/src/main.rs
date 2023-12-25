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
    let address = "127.0.0.1:8080";
    println!("server run on:{}", address);
    println!("server pid:{}", process::id()); // 服务启动的进程id

    // with_state 共享数据
    let shared_state = Arc::new(handlers::AppState::default());
    // 创建axum router
    let router = Router::new()
        .route("/:key", get(handlers::short_url))
        .with_state(shared_state.clone())
        .route(
            "/create",
            post({
                // 通过闭包的形式传递shared_state数据
                let shared_state = shared_state.clone();
                move |body| handlers::create(body, shared_state)
            }),
        )
        .fallback(handlers::not_found_handler);

    // 创建axum app实例并启动服务
    let addr: SocketAddr = address.parse().unwrap();
    axum::Server::bind(&addr)
        .serve(router.into_make_service()) // 将router转化为service
        .with_graceful_shutdown(handlers::graceful_shutdown()) // 设置平滑退出函数
        .await
        .unwrap();
}
