use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::RwLock;

/// graceful shutdown operation
// graceful shutdown
// When the exit semaphore is received,
// the f function that the service needs to execute before exiting
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

// job平滑退出处理，它是通过接收退出信号量的方式完成，
// 当接收到退出信号量时，就将stop原子引用类型所对应的异步写锁的值设置为true
// stop是一个引用计数bool类型的异步读写锁
pub async fn job_graceful_shutdown(wait_time: Duration, stop: Arc<RwLock<bool>>) {
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

    // 监听退出信号量
    tokio::select! {
        _ = ctrl_c =>{
            println!("received ctrl_c signal,server will exist...");
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
        },
    }

    // 平滑退出等待时长
    tokio::time::sleep(wait_time).await;

    println!("signal received,starting graceful shutdown");

    // 将stop赋值为true，表示接收到退出信号量
    let mut exit = stop.write().await;
    *exit = true
}
