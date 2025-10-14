// 引入axum包
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;

// 用于json序列化处理
use serde::{Deserialize, Serialize};
use std::process;
use std::time::Duration;
use tokio::signal; // 用于signal平滑退出

#[tokio::main]
async fn main() {
    // HTTP服务运行地址
    let address = "127.0.0.1:8080";
    println!("server run on:{}", address);
    println!("server pid:{}", process::id()); // 服务启动的进程id

    // 创建axum router
    let router = Router::new()
        .route("/hello", get(hello))
        .route("/foo", post(foo))
        .fallback(not_found_handler);

    // 创建一个listener对象
    let addr: SocketAddr = address.parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // 启动HTTP服务
    axum::serve(listener, router)
        .with_graceful_shutdown(graceful_shutdown()) // 设置平滑退出函数
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    println!("hello");
    "hello,world"
}

// 处理路由找不到的情况
async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "this page not found")
}

// 通过derive标注实现json序列化处理
#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
    id: i64,
}

// 返回json格式
async fn foo() -> impl IntoResponse {
    let cat = Cat {
        id: 1,
        name: "xiaoming".to_string(),
    };

    // Json在axum底层实际类型：pub struct Json<T>(pub T);
    // 它实现了IntoResponse trait，所以这里可以直接使用Json(cat)返回结果
    Json(cat)
}

// 接收signal信号量并平滑退出
async fn graceful_shutdown() {
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

    let graceful_wait_time = Duration::from_secs(5); // 平滑退出等待时间

    // 对于非unix平台，通过cfg标记属性来定义terminate退出机制
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c =>{
            println!("received ctrl_c signal,server will exist...");
            tokio::time::sleep(graceful_wait_time).await;
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
            tokio::time::sleep(graceful_wait_time).await;
        },
    }

    println!("signal received,starting graceful shutdown");
}
