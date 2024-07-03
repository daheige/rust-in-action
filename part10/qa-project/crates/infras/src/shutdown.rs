use std::sync::mpsc;
use std::time::Duration;
use tokio::signal;

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

    tokio::time::sleep(wait_time).await;
    println!("service graceful shutdown success");
}

// job平滑退出信号量处理
pub async fn job_graceful_shutdown(wait_time: Duration, sender: mpsc::Sender<bool>) {
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

    // 监听退出信号量
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c =>{
            println!("received ctrl_c signal,server will exist...");
            tokio::time::sleep(wait_time).await;
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
            tokio::time::sleep(wait_time).await;
        },
    }

    println!("signal received,starting graceful shutdown");

    // 发送退出通知
    sender.send(true).unwrap();
}
