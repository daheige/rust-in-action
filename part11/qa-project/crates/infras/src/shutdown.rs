use std::time::Duration;
use tokio::signal; // 引入tokio signal模块

// 平滑退出处理，它是通过接收退出信号量的方式完成，
// 当接收到退出信号量时，程序会等待一段时间，再退出。
// 在这个退出期间，程序可以做一些其他的清理操作
pub async fn graceful_shutdown(wait_time: Duration) {
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

    // 监听退出信号量
    tokio::select! {
        _ = ctrl_c =>{
            println!("received ctrl_c signal,server will exist...");
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
        },
    }

    // 平滑退出等待
    tokio::time::sleep(wait_time).await;
    println!("service graceful shutdown success");
}
