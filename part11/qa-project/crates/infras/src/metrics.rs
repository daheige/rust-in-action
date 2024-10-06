use super::graceful_shutdown;
use autometrics::prometheus_exporter;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;

// prometheus metrics初始化配置
pub async fn prometheus_init(port: u16) {
    // Set up prometheus metrics exporter
    prometheus_exporter::init();

    // Build http /metrics endpoint
    let router = Router::new().route(
        "/metrics",
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    let address: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    let listener = TcpListener::bind(address).await.unwrap();
    println!("prometheus at:{}/metrics", address);

    // Start http service
    axum::serve(listener, router)
        .with_graceful_shutdown(graceful_shutdown(Duration::from_secs(5)))
        .await
        .expect("prometheus metrics init failed");
}
